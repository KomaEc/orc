mod rewrite_body;

use rustc_hir::{def_id::LocalDefId, FnRetTy, FnSig, ItemKind, Ty, TyKind};
use rustc_middle::{
    mir::{Field, Local},
    ty::TyCtxt,
};

use orc_common::AnalysisResults;
use usage_analysis::{fatness, mutability, null};

use crate::rewriter::{RewriteMode, Rewriter};
use rewrite_body::{rewrite_body, BodyRewriteCtxt};

fn ty_nested_depth(ty: &Ty) -> usize {
    match &ty.kind {
        TyKind::Ptr(mut_ty) => 1 + ty_nested_depth(mut_ty.ty),
        _ => 0,
    }
}

pub fn rewrite<'tcx, 'a>(
    tcx: TyCtxt<'tcx>,
    orc_ownership_analysis: &dyn AnalysisResults,
    mutability_analysis: &mutability::CrateResults<'tcx, 'a>,
    fatness_analysis: &fatness::CrateResults<'tcx, 'a>,
    null_analysis: &null::CrateResults<'tcx, 'a>,
    fn_defs: &[LocalDefId],
    struct_defs: &[LocalDefId],
    rewrite_mode: RewriteMode,
) {
    let mut rewriter = Rewriter::default();
    rewrite_structs(
        tcx,
        &mut rewriter,
        orc_ownership_analysis,
        fatness_analysis,
        null_analysis,
        struct_defs,
    );
    rewrite_functions(
        tcx,
        &mut rewriter,
        orc_ownership_analysis,
        mutability_analysis,
        fatness_analysis,
        null_analysis,
        fn_defs,
    );

    rewriter.write(rewrite_mode)
}

fn rewrite_functions<'tcx, 'a>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    orc_ownership_analysis: &dyn AnalysisResults,
    mutability_analysis: &mutability::CrateResults<'tcx, 'a>,
    fatness_analysis: &fatness::CrateResults<'tcx, 'a>,
    null_analysis: &null::CrateResults<'tcx, 'a>,
    fn_defs: &[LocalDefId],
) {
    for &did in fn_defs {
        let item = tcx.hir().expect_item(did);
        let ItemKind::Fn(sig, _generics, _body_id) = &item.kind else { panic!() };
        rewrite_fn_sig(
            tcx,
            rewriter,
            orc_ownership_analysis,
            mutability_analysis,
            fatness_analysis,
            null_analysis,
            did,
            sig,
        );
        let mut body_rewrite_cx = BodyRewriteCtxt {
            tcx,
            rewriter,
            ownership: orc_ownership_analysis,
            mutability: mutability_analysis,
            fatness: fatness_analysis,
            null: null_analysis,
            def_id: did,
            body: tcx.optimized_mir(did),
        };
        rewrite_body(&mut body_rewrite_cx);
    }
}

fn rewrite_fn_sig(
    tcx: TyCtxt,
    rewriter: &mut Rewriter,
    ownership: &dyn AnalysisResults,
    mutability: &mutability::CrateResults,
    fatness: &fatness::CrateResults,
    null: &null::CrateResults,
    def_id: LocalDefId,
    sig: &FnSig,
) {
    let results_for_local = |i, ty| {
        let local = Local::from_usize(i);
        let nested_depth = ty_nested_depth(ty);
        (0..nested_depth)
            .map(|nested_level| PtrResults {
                owning: ownership.sig_result(def_id, local, nested_level),
                fat: fatness
                    .sig_result(def_id, local, nested_level)
                    .unwrap_or(false),
                // i thought this unwrap_or should be true, but using false causes fewer errors in
                // bst-good :)
                mutable: mutability
                    .sig_result(def_id, local, 0 /* TODO */)
                    .unwrap_or(false),
                nullable: null.local_result(def_id, local, nested_level).unwrap(),
            })
            .collect::<Vec<_>>()
    };

    if let FnRetTy::Return(ty) = sig.decl.output {
        rewrite_raw_ptr_ty(tcx, rewriter, ty, &results_for_local(0, ty));
    }

    for (i, ty) in sig.decl.inputs.iter().enumerate() {
        rewrite_raw_ptr_ty(tcx, rewriter, ty, &results_for_local(i + 1, ty));
    }
}

fn rewrite_structs<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    orc_ownership_analysis: &dyn AnalysisResults,
    fatness_analysis: &fatness::CrateResults<'tcx, '_>,
    null_analysis: &null::CrateResults<'tcx, '_>,
    dids: &[LocalDefId],
) {
    for &did in dids {
        let item = tcx.hir().expect_item(did);
        let ItemKind::Struct(variant_data, _generics) = &item.kind else { panic!() };
        rewrite_struct(
            tcx,
            rewriter,
            orc_ownership_analysis,
            fatness_analysis,
            null_analysis,
            variant_data,
            did,
        );
    }
}

fn rewrite_struct<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    ownership: &dyn AnalysisResults,
    fatness: &fatness::CrateResults<'tcx, '_>,
    null: &null::CrateResults<'tcx, '_>,
    variant_data: &rustc_hir::VariantData,
    did: LocalDefId,
) {
    for (idx, field) in variant_data.fields().iter().enumerate() {
        let field_idx = Field::from_usize(idx);
        let nested_depth = ty_nested_depth(field.ty);
        let results = (0..nested_depth)
            .map(|nested_level| {
                PtrResults {
                    owning: ownership.field_result(did, field_idx, nested_level),
                    fat: fatness.field_result(did, field_idx, nested_level).unwrap(),
                    mutable: true, // TODO
                    nullable: null.field_result(did, field_idx, nested_level).unwrap(),
                }
            })
            .collect::<Vec<_>>();
        rewrite_raw_ptr_ty(tcx, rewriter, field.ty, &results);
    }
}

#[derive(Clone, Copy)]
struct PtrResults {
    owning: Option<bool>,
    fat: bool,
    mutable: bool,
    nullable: bool,
}

fn rewrite_raw_ptr_ty(tcx: TyCtxt<'_>, rewriter: &mut Rewriter, ty: &Ty, results: &[PtrResults]) {
    // we want both recursion and local variable capture, so we need both a fn and a closure
    fn visit_nested_pointer(
        ty: &Ty,
        results: &[PtrResults],
        f: &mut impl FnMut(&Ty, Option<&PtrResults>),
    ) {
        if let TyKind::Ptr(mut_ty) = &ty.kind {
            visit_nested_pointer(mut_ty.ty, &results[1..], f);
        }
        f(ty, results.get(0));
    }

    let mut new_ty = String::new();
    visit_nested_pointer(ty, results, &mut |ty, result| {
        if !matches!(ty.kind, TyKind::Ptr(_)) {
            new_ty = tcx.sess.source_map().span_to_snippet(ty.span).unwrap();
            return;
        }
        let result = result.unwrap();

        if result.fat {
            new_ty = format!("[{new_ty}]");
        }

        let ptr = match (result.owning, result.mutable) {
            (Some(true), _) => {
                new_ty.push('>');
                "Box<"
            }
            (Some(false), true) => "&mut ",
            (Some(false), false) => "&",
            (None, true) => "*mut ",
            (None, false) => "*const ",
        };
        new_ty.insert_str(0, ptr);

        if result.nullable {
            new_ty.insert_str(0, "Option<");
            new_ty.push('>');
        }
    });
    rewriter.make_suggestion(tcx, ty.span, String::new(), new_ty);
}