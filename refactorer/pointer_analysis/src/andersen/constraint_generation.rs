use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{
    Body, Local, LocalDecl, Location, NullOp, Place, Rvalue, Terminator, TerminatorKind,
    RETURN_PLACE,
};
use rustc_middle::mir::{CastKind, Operand, ProjectionElem};
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TyKind::FnDef;

use crate::{
    andersen::constraint_solving::ConstraintSolving,
    andersen::ctxt::AndersenAnalysisCtxt,
    andersen::{AndersenNode, Constraint, ConstraintKind, ConstraintSet},
};

/// Data structure for constraint generation.
/// 'cg = the duration of the constraint generation
pub struct ConstraintGeneration<'cg, 'tcx> {
    constraints: ConstraintSet,
    aa_ctxt: AndersenAnalysisCtxt<'cg, 'tcx>,
}

impl<'cg, 'tcx> ConstraintGeneration<'cg, 'tcx> {
    pub fn new(
        all_functions: &'cg [LocalDefId],
        tcx: TyCtxt<'tcx>,
    ) -> ConstraintGeneration<'cg, 'tcx> {
        ConstraintGeneration {
            constraints: ConstraintSet::new(),
            aa_ctxt: AndersenAnalysisCtxt::new(all_functions, tcx),
        }
    }

    #[inline]
    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.aa_ctxt.tcx()
    }

    pub fn generate_constraints(mut self) -> Self {
        for &did in self.aa_ctxt.all_functions {
            let (body, _) = self
                .tcx()
                .mir_promoted(rustc_middle::ty::WithOptConstParam::unknown(did));
            let body_ref = body.borrow();
            ConstraintGenerationForBody {
                func_cx: did,
                body: &*body_ref,
                aa_ctxt: &mut self.aa_ctxt,
                constraints: &mut self.constraints,
            }
            .visit_body(&*body_ref);
        }
        self.log_debug_constraints();
        self
    }

    pub fn proceed_to_solving(self) -> ConstraintSolving<'cg, 'tcx> {
        ConstraintSolving::new(self.constraints, self.aa_ctxt)
    }

    fn log_debug_constraints(&self) {
        log::debug!("Dumping constraints:");
        for constraint in self.constraints.iter() {
            let lhs = self.aa_ctxt.node_to_str(constraint.left);
            let rhs = self.aa_ctxt.node_to_str(constraint.right);
            match constraint.constraint_kind {
                ConstraintKind::AddressOf => log::debug!("{} = &{}", lhs, rhs),
                ConstraintKind::Copy => log::debug!("{} = {}", lhs, rhs),
                ConstraintKind::Load => log::debug!("{} = *{}", lhs, rhs),
                ConstraintKind::Store => log::debug!("*{} = {}", lhs, rhs),
            }
        }
    }
}

struct ConstraintGenerationForBody<'me, 'cg, 'tcx> {
    func_cx: LocalDefId,
    body: &'me Body<'tcx>,
    aa_ctxt: &'me mut AndersenAnalysisCtxt<'cg, 'tcx>,
    constraints: &'me mut ConstraintSet,
}

impl<'me, 'cg, 'tcx> Visitor<'tcx> for ConstraintGenerationForBody<'me, 'cg, 'tcx> {
    /// Default visitor will visit basic blocks before local declarations,
    /// so we overwrite here.
    fn visit_body(&mut self, body: &Body<'tcx>) {
        log::trace!("visiting body");
        for (local, decl) in body.local_decls.iter_enumerated() {
            self.visit_local_decl(local, decl)
        }

        for (bb, data) in body.basic_blocks().iter_enumerated() {
            self.visit_basic_block_data(bb, data)
        }
    }

    fn visit_local_decl(&mut self, local: Local, local_decl: &LocalDecl<'tcx>) {
        log::trace!("visiting local declaration {:?} : {}", local, local_decl.ty);
        let LocalDecl {
            mutability: _,
            ty,
            user_ty: _,
            source_info: _,
            internal: _,
            local_info: _,
            is_block_tail: _,
        } = local_decl;

        if ty.is_any_ptr() {
            if ty.is_fn_ptr() {
                log::error!("Function pointer: {} is not supported!", ty);
                unimplemented!()
            }
            // generate andersen node for this local
            let _ = self.aa_ctxt.generate_from_local(self.func_cx, local);
        }

        // self.super_local_decl(local, local_decl)
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        log::trace!(
            "visiting assignment statment {:?} = {:?} at location: {:?}",
            place,
            rvalue,
            location
        );

        let place_ty = place.ty(self.body, self.aa_ctxt.tcx());
        if place_ty.ty.is_any_ptr() {
            if place_ty.ty.is_fn_ptr() {
                log::error!("Function pointer: {} is not supported!", place_ty.ty);
                unimplemented!()
            }

            // self.process_assign_of_ptr_ty(place, rvalue, location);

            let (rhs_repr, rhs_ptr_kind) = self.process_rvalue_of_ptr_ty(rvalue, location);
            let (lhs_repr, lhs_is_indirect) = self.process_place_of_ptr_ty(place, location);

            match rhs_ptr_kind {
                RhsPtrKind::Use {
                    indirect: rhs_is_indirect,
                } => {
                    match (lhs_is_indirect, rhs_is_indirect) {
                        // *p = *q, introduce a temporary
                        // tmp = *q
                        // *p = tmp
                        (true, true) => {
                            let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                            /// TODO: tmp.load(rhs_repr);
                            self.constraints.push(Constraint::new(
                                ConstraintKind::Load,
                                tmp,
                                rhs_repr,
                            ));
                            self.constraints.push(Constraint::new(
                                ConstraintKind::Store,
                                lhs_repr,
                                tmp,
                            ));
                        }
                        // *p = q
                        (true, false) => {
                            self.constraints.push(Constraint::new(
                                ConstraintKind::Store,
                                lhs_repr,
                                rhs_repr,
                            ));
                        }
                        // p = *q
                        (false, true) => {
                            self.constraints.push(Constraint::new(
                                ConstraintKind::Load,
                                lhs_repr,
                                rhs_repr,
                            ));
                        }
                        // p = q
                        (false, false) => {
                            self.constraints.push(Constraint::new(
                                ConstraintKind::Copy,
                                lhs_repr,
                                rhs_repr,
                            ));
                        }
                    }
                }
                RhsPtrKind::AddressOf => {
                    match lhs_is_indirect {
                        // *p = &q, introduce a temporary
                        // tmp = &q
                        // *p = tmp
                        true => {
                            let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                            self.constraints.push(Constraint::new(
                                ConstraintKind::AddressOf,
                                tmp,
                                rhs_repr,
                            ));
                            self.constraints.push(Constraint::new(
                                ConstraintKind::Store,
                                lhs_repr,
                                tmp,
                            ));
                        }
                        // p = &q
                        false => {
                            self.constraints.push(Constraint::new(
                                ConstraintKind::AddressOf,
                                lhs_repr,
                                rhs_repr,
                            ));
                        }
                    }
                }
            }
        }

        // self.super_assign(place, rvalue, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        let Terminator {
            source_info: _,
            kind,
        } = terminator;
        if let TerminatorKind::Call {
            func,
            args,
            destination,
            cleanup: _,
            from_hir_call,
            fn_span: _,
        } = kind
        {
            if let Some((place, _)) = destination {
                let place_ty = place.ty(self.body, self.aa_ctxt.tcx());
                if place_ty.ty.is_any_ptr() {
                    if place_ty.ty.is_fn_ptr() {
                        unimplemented!("Function pointer")
                    }
                    // processing terminator `p = f(q1, q2, ..)`
                    if let FnDef(def_id, generic_args) = func.constant().unwrap().ty().kind() {
                        // process lhs
                        let (lhs_repr, p_is_indirect) =
                            self.process_place_of_ptr_ty(place, location);

                        if let Some(callee) = def_id.as_local() {
                            assert!(
                                generic_args.is_empty(),
                                "Generic functions are not supported"
                            );
                            assert!(from_hir_call, "Inner functions are not supported");
                            assert!(self.aa_ctxt.all_functions.contains(&callee));

                            // The purpose of getting the callee body is to obtain the argument local list of the callee.
                            // However, if we can cheat if we know the calling convention of mir.
                            // From the doc, we know that the arguments starts from local 1.
                            log::warn!("Calling convention of MIR");

                            // generate constraint: `p = f.RETURN_PLACE`
                            /// FIXME: really??? we have to generate return place before analysis!
                            let ret_repr = self
                                .aa_ctxt
                                .lookup_local(callee, RETURN_PLACE)
                                .expect("the return place must have been generated.");
                            self.constraints.push(Constraint::new(
                                if p_is_indirect {
                                    ConstraintKind::Store
                                } else {
                                    ConstraintKind::Copy
                                },
                                lhs_repr,
                                ret_repr,
                            ));

                            // For operands p1, ..., pn, and arguments of callee q1, ... qn,
                            // generate constraints q1 = p1, ..., qn = pn
                            for (i, operand) in args.iter().enumerate() {
                                let local = Local::from_usize(i + 1);
                                if let Some(place) = operand.place() {
                                    let place_ty = place.ty(self.body, self.aa_ctxt.tcx());
                                    if place_ty.ty.is_any_ptr() {
                                        if place_ty.ty.is_fn_ptr() {
                                            unimplemented!("Function pointer")
                                        }
                                        let lhs_repr = self
                                            .aa_ctxt
                                            .lookup_local(callee, local)
                                            .expect("argument places must have been generated");
                                        let (rhs_repr, rhs_is_indirect) = self
                                            .process_place_of_ptr_ty(
                                                &operand.place().unwrap(),
                                                location,
                                            );

                                        if rhs_is_indirect {
                                            self.constraints.push(Constraint::new(
                                                ConstraintKind::Load,
                                                lhs_repr,
                                                rhs_repr,
                                            ));
                                        } else {
                                            self.constraints.push(Constraint::new(
                                                ConstraintKind::Copy,
                                                lhs_repr,
                                                rhs_repr,
                                            ));
                                        }
                                    }
                                }
                            }
                        } else {
                            log::error!("UNIMPLEMENTED: model library functions")
                        }
                    }
                }
            }
        }
    }
}

impl<'me, 'cg, 'tcx> ConstraintGenerationForBody<'me, 'cg, 'tcx> {
    /// Process place of pointer type, return an Andersen node representing this place. Return true
    /// if this place is indirect
    ///
    /// If `place` is nested, for instance, `*(*(*p).0).1`, introduce temporary variables implicitly.
    /// In the above example, temp vars are introduced as:
    /// ```mir
    /// tmp1 = *p;
    /// tmp2 = *tmp1;
    /// *tmp2 = ... // or ... = *tmp2
    /// ```
    /// , and `(andersen_repr(tmp2), true)` is returned
    /// Note that, the current analysis is field insensitive, meaning that assignment to `x.f` is treated
    /// the same as assignment to `x`.
    fn process_place_of_ptr_ty(
        &mut self,
        place: &Place<'tcx>,
        location: Location,
    ) -> (AndersenNode, bool) {
        log::trace!("processing place {:?} at location {:?}", place, location);

        //for (place_ref, _) in place.iter_projections() {
        //    let _ = self.node_generation.generate(place_ref.into());
        //}

        let mut repr = self.aa_ctxt.generate_from_local(self.func_cx, place.local);
        let mut is_nested = false;
        let mut is_indirect = false;

        for (_place_ref, proj_elem) in place.iter_projections() {
            match proj_elem {
                ProjectionElem::Deref => {
                    is_indirect = true;
                    if is_nested {
                        let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                        // generate constraint: p = *tmp
                        self.constraints
                            .push(Constraint::new(ConstraintKind::Load, repr, tmp));
                        repr = tmp;
                    } else {
                        is_nested = true;
                    }
                }
                ProjectionElem::Field(f, _) => {
                    log::warn!("field {:?} ignored!", f);
                }
                ProjectionElem::Index(_) => unimplemented!("projection: index"),
                ProjectionElem::ConstantIndex {
                    offset: _,
                    min_length: _,
                    from_end: _,
                } => unimplemented!("projection: const index"),
                ProjectionElem::Subslice {
                    from: _,
                    to: _,
                    from_end: _,
                } => unimplemented!("projection: subslice"),
                ProjectionElem::Downcast(_, _) => unimplemented!("projection: downcast"),
            }
        }
        (repr, is_indirect)
    }

    fn process_rvalue_of_ptr_ty(
        &mut self,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) -> (AndersenNode, RhsPtrKind) {
        match rvalue {
            Rvalue::Use(Operand::Copy(place))
            | Rvalue::Use(Operand::Move(place))
            | Rvalue::Cast(CastKind::Pointer(_), Operand::Copy(place), _)
            | Rvalue::Cast(CastKind::Pointer(_), Operand::Move(place), _) => {
                let (rhs_repr, rhs_is_indirect) = self.process_place_of_ptr_ty(place, location);

                (
                    rhs_repr,
                    RhsPtrKind::Use {
                        indirect: rhs_is_indirect,
                    },
                )
            }

            Rvalue::Ref(_, _, place) | Rvalue::AddressOf(_, place) => {
                let (mut rhs_repr, rhs_is_indirect) = self.process_place_of_ptr_ty(place, location);
                // ... = &*q, introduce a temporary
                // tmp = *q
                // ... = &tmp
                if rhs_is_indirect {
                    let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                    self.constraints
                        .push(Constraint::new(ConstraintKind::Load, tmp, rhs_repr));
                    rhs_repr = tmp;
                }

                (rhs_repr, RhsPtrKind::AddressOf)
            }

            Rvalue::NullaryOp(NullOp::Box, _ty) => {
                log::error!("Box::new() is not supported!");
                unimplemented!()
            }

            _ => {
                log::error!("rvalue of this kind: {:?} is not supported!", rvalue);
                unimplemented!()
            }
        }
    }

    #[allow(dead_code)]
    fn process_assign_of_ptr_ty(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        let (lhs_repr, lhs_is_indirect) = self.process_place_of_ptr_ty(place, location);
        match rvalue {
            Rvalue::Use(Operand::Copy(place))
            | Rvalue::Use(Operand::Move(place))
            | Rvalue::Cast(CastKind::Pointer(_), Operand::Copy(place), _)
            | Rvalue::Cast(CastKind::Pointer(_), Operand::Move(place), _) => {
                let (rhs_repr, rhs_is_indirect) = self.process_place_of_ptr_ty(place, location);
                match (lhs_is_indirect, rhs_is_indirect) {
                    // *p = *q, introduce a temporary
                    // tmp = *q
                    // *p = tmp
                    (true, true) => {
                        let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                        self.constraints
                            .push(Constraint::new(ConstraintKind::Load, tmp, rhs_repr));
                        self.constraints.push(Constraint::new(
                            ConstraintKind::Store,
                            lhs_repr,
                            tmp,
                        ));
                    }
                    // *p = q
                    (true, false) => {
                        self.constraints.push(Constraint::new(
                            ConstraintKind::Store,
                            lhs_repr,
                            rhs_repr,
                        ));
                    }
                    // p = *q
                    (false, true) => {
                        self.constraints.push(Constraint::new(
                            ConstraintKind::Load,
                            lhs_repr,
                            rhs_repr,
                        ));
                    }
                    // p = q
                    (false, false) => {
                        self.constraints.push(Constraint::new(
                            ConstraintKind::Copy,
                            lhs_repr,
                            rhs_repr,
                        ));
                    }
                }
            }

            Rvalue::Ref(_, _, place) | Rvalue::AddressOf(_, place) => {
                let (mut rhs_repr, rhs_is_indirect) = self.process_place_of_ptr_ty(place, location);
                // ... = &*q, introduce a temporary
                // tmp = *q
                // ... = &tmp
                if rhs_is_indirect {
                    let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                    self.constraints
                        .push(Constraint::new(ConstraintKind::Load, tmp, rhs_repr));
                    rhs_repr = tmp;
                }

                match lhs_is_indirect {
                    // *p = &q, introduce a temporary
                    // tmp = &q
                    // *p = tmp
                    true => {
                        let tmp = self.aa_ctxt.generate_temporary(self.func_cx);
                        self.constraints.push(Constraint::new(
                            ConstraintKind::AddressOf,
                            tmp,
                            rhs_repr,
                        ));
                        self.constraints.push(Constraint::new(
                            ConstraintKind::Store,
                            lhs_repr,
                            tmp,
                        ));
                    }
                    // p = &q
                    false => {
                        self.constraints.push(Constraint::new(
                            ConstraintKind::AddressOf,
                            lhs_repr,
                            rhs_repr,
                        ));
                    }
                }
            }

            Rvalue::NullaryOp(NullOp::Box, _ty) => {
                log::error!("Box::new() is not supported!");
                unimplemented!()
            }

            _ => {
                log::error!("rvalue of this kind: {:?} is not supported!", rvalue);
                unimplemented!()
            }
        }
    }
}

enum RhsPtrKind {
    AddressOf,
    Use { indirect: bool },
}