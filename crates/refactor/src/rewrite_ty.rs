use common::rewrite::Rewrite;
use rustc_hir::{def_id::DefId, Item, ItemKind};
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;

use crate::{HirExt, PointerData, PointerKind, StructDecision};

pub fn rewrite_structs(
    structs: &[DefId],
    struct_decision: &StructDecision,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
) -> anyhow::Result<()> {
    use std::fmt::Write;
    for did in structs {
        let fields_data = struct_decision.fields_data(did);
        let item = tcx.hir().expect_item(did.expect_local());
        let mut default_impl_block = String::new();
        writeln!(
            default_impl_block,
            "\nimpl Default for {} {{fn default() -> Self {{Self {{",
            item.ident
        )?;
        rewrite_struct(item, fields_data, &mut default_impl_block, rewriter, tcx)?;

        writeln!(default_impl_block, "}}}}}}").unwrap();

        let struct_span = item.span;
        rewriter.replace(tcx, struct_span.shrink_to_hi(), default_impl_block);
    }
    Ok(())
}

pub fn rewrite_struct(
    r#struct: &Item,
    decision: &[SmallVec<[PointerData; 3]>],
    default_impl_body: &mut String,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let ItemKind::Struct(variant_data, _generics) = &r#struct.kind else { panic!() };
    for (field, decision) in itertools::izip!(variant_data.fields(), decision) {
        for (raw_ptr_ty, decision) in itertools::izip!(field.ty.walk_ptr(), decision) {
            rewrite_raw_ptr(raw_ptr_ty, decision.pointer_kind, rewriter, tcx);
        }

        if let rustc_hir::TyKind::Ptr(pointee) = &field.ty.kind {
            let decision = decision.first().unwrap();
            if decision.pointer_kind.is_raw() {
                match pointee.mutbl {
                    rustc_ast::Mutability::Mut => {
                        writeln!(default_impl_body, "{}: std::ptr::null_mut(),", field.ident)?;
                    }
                    rustc_ast::Mutability::Not => {
                        writeln!(default_impl_body, "{}: std::ptr::null(),", field.ident)?;
                    }
                }
            } else {
                writeln!(default_impl_body, "{}: None,", field.ident)?;
            }
        } else {
            writeln!(default_impl_body, "{}: Default::default(),", field.ident)?;
        }
    }

    Ok(())
}

pub fn rewrite_raw_ptr(
    ty: &rustc_hir::Ty,
    decision: PointerKind,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
) {
    let rustc_hir::TyKind::Ptr(pointee) = &ty.kind else { unreachable!() };

    match decision {
        PointerKind::Move => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<Box<".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">>".to_owned());
        }
        PointerKind::Mut => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<&mut".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">".to_owned());
        }
        PointerKind::Shr => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<&".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">".to_owned());
        }
        PointerKind::Raw => {}
    }
}