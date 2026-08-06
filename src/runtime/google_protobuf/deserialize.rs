 /**
  * Copyright 2024 ByteDance and/or its affiliates
  *
  * Original Files：protoc-gen-ts (https://github.com/thesayyn/protoc-gen-ts)
  * Copyright (c) 2024 Sahin Yort
  * SPDX-License-Identifier: MIT 
 */

use super::GooglePBRuntime;
use crate::common::field;
use crate::descriptor::field_descriptor_proto;
use crate::{context::Context, descriptor};

use std::vec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
    AssignOp, BinaryOp, BlockStmt, BreakStmt, Expr, ParenExpr, PatOrExpr, Stmt, SwitchCase,
    SwitchStmt, ThrowStmt, TsArrayType, TsAsExpr, TsKeywordType, TsKeywordTypeKind, TsNonNullExpr,
    TsType, WhileStmt,
};
use swc_ecma_utils::{quote_ident, quote_str};

impl GooglePBRuntime {
    pub(super) fn deserialize_setup_inner(
        &self,
        ctx: &mut Context,
        descriptor: &descriptor::DescriptorProto,
        create_br: bool,
    ) -> Vec<Stmt> {
        let mut stmts = vec![];

        if create_br {
            ctx.get_protobuf_import(&ctx.options.runtime_package);
            let br_decl_init = crate::new_expr!(
                Expr::Ident(quote_ident!("BinaryReader")),
                vec![crate::expr_or_spread!(quote_ident!("bytes").into())]
            );

            let br_decl = Stmt::Decl(
                crate::const_decl!(format!("{}: BinaryReader", "br"),
                br_decl_init));
            stmts.push(br_decl)
        }

        stmts.push(self.deserialize_stmt(ctx, descriptor, field::this_field_member, true));

        stmts
    }

    fn deserialize_message_field_preread_expr(
        &self,
        ctx: &mut Context,
        field: &descriptor::FieldDescriptorProto,
        accessor: field::FieldAccessorFn,
    ) -> Expr {
        crate::assign_expr!(
            PatOrExpr::Expr(Box::new(accessor(field))),
            crate::new_expr!(ctx.lazy_type_ref(field.type_name()).into()),
            AssignOp::NullishAssign
        )
    }

    fn deserialize_message_field_expr(
        &self,
        ctx: &mut Context,
        field: &descriptor::FieldDescriptorProto,
        accessor: field::FieldAccessorFn,
    ) -> Expr {
        let member_expr = if field.is_repeated() {
            crate::member_expr!(ctx.lazy_type_ref(field.type_name()), "fromBinary")
        } else {
            crate::member_expr_bare!(accessor(field).into(), "mergeFrom")
        };
        crate::call_expr!(
            member_expr,
            vec![crate::expr_or_spread!(crate::call_expr!(
                crate::member_expr!("br", "readBytes")
            ))]
        )
    }

    fn deserialize_primitive_field_expr(
        &self,
        field: &descriptor::FieldDescriptorProto,
        packed: bool,
    ) -> Expr {
        let mut call = crate::call_expr!(crate::member_expr!(
            "br",
            self.rw_function_name_for_encoding("read", field, packed)
        ));
        if packed {
            let element_kind = if field.is_bigint() {
                TsKeywordTypeKind::TsStringKeyword
            } else {
                TsKeywordTypeKind::TsNumberKeyword
            };
            call = Expr::Paren(ParenExpr {
                span: DUMMY_SP,
                expr: Box::new(Expr::TsAs(TsAsExpr {
                    span: DUMMY_SP,
                    expr: Box::new(call),
                    type_ann: Box::new(TsType::TsArrayType(TsArrayType {
                        span: DUMMY_SP,
                        elem_type: Box::new(TsType::TsKeywordType(TsKeywordType {
                            span: DUMMY_SP,
                            kind: element_kind,
                        })),
                    })),
                })),
            });
        }
        if packed && field.is_bigint() {
            call = crate::call_expr!(
                crate::member_expr_bare!(call, "map"),
                vec![crate::expr_or_spread!(crate::arrow_func_short!(
                    crate::call_expr!(
                        quote_ident!("BigInt").into(),
                        vec![crate::expr_or_spread!(quote_ident!("r").into())]
                    ),
                    vec![crate::pat_ident!(quote_ident!("r: string"))]
                ))]
            );
        } else if packed && field.is_booelan() {
            call = crate::call_expr!(
                crate::member_expr_bare!(call, "map"),
                vec![crate::expr_or_spread!(crate::arrow_func_short!(
                    crate::bin_expr!(
                        Expr::Ident(quote_ident!("r")),
                        crate::lit_num!(0).into(),
                        BinaryOp::NotEqEq
                    ),
                    vec![crate::pat_ident!(quote_ident!("r: number"))]
                ))]
            );
        } else if field.is_bigint() {
            call = crate::call_expr!(
                quote_ident!("BigInt").into(),
                vec![crate::expr_or_spread!(call)]
            );
        } else if !packed && field.type_() == field_descriptor_proto::Type::TYPE_UINT32 {
            call = crate::bin_expr!(call, crate::lit_num!(0).into(), BinaryOp::ZeroFillRShift)
        } else if field.is_booelan() {
            call = crate::bin_expr!(call, crate::lit_num!(0).into(), BinaryOp::NotEqEq)
        }
        call
    }

    fn deserialize_map_field_expr(
        &self,
        ctx: &mut Context,
        field: &descriptor::FieldDescriptorProto,
        accessor: field::FieldAccessorFn,
    ) -> Expr {
        let descriptor = ctx
            .get_map_type(field.type_name())
            .expect(format!("can not find the map type {}", field.type_name()).as_str());
        let key_field = &descriptor.field[0];
        let value_field = &descriptor.field[1];

        crate::call_expr!(
            crate::member_expr!("br", "readMessage"),
            vec![
                crate::expr_or_spread!(quote_ident!("undefined").into()),
                crate::expr_or_spread!(crate::arrow_func!(
                    vec![],
                    vec![
                        Stmt::Decl(crate::let_decl!(
                            "key",
                            key_field.type_annotation(ctx),
                            key_field.default_value_expr(ctx, true)
                        )),
                        Stmt::Decl(crate::let_decl!(
                            "value",
                            value_field.type_annotation(ctx),
                            value_field.default_value_expr(ctx, true)
                        )),
                        self.deserialize_stmt(ctx, &descriptor, field::bare_field_member, false),
                        crate::expr_stmt!(crate::call_expr!(
                            crate::member_expr_bare!(crate::member_expr!("this", format!("{}?", field.name())), "set"),
                            vec![
                                crate::expr_or_spread!(Expr::TsNonNull(TsNonNullExpr {
                                    expr: Box::new(Expr::Ident(quote_ident!("key"))),
                                    span: DUMMY_SP
                                })),
                                crate::expr_or_spread!(Expr::TsNonNull(TsNonNullExpr {
                                    expr: Box::new(Expr::Ident(quote_ident!("value"))),
                                    span: DUMMY_SP
                                })),
                            ]
                        ))
                    ]
                ))
            ]
        )
    }

    fn deserialize_field_expr(
        &self,
        ctx: &mut Context,
        field: &descriptor::FieldDescriptorProto,
        accessor: field::FieldAccessorFn,
        packed: bool,
    ) -> Expr {
        if field.is_map(ctx) {
            self.deserialize_map_field_expr(ctx, field, accessor)
        } else if field.is_message() {
            self.deserialize_message_field_expr(ctx, field, accessor)
        } else {
            self.deserialize_primitive_field_expr(field, packed)
        }
    }

    pub fn deserialize_stmt(
        &self,
        ctx: &mut Context,
        descriptor: &descriptor::DescriptorProto,
        accessor: field::FieldAccessorFn,
        add_unknown_fields: bool,
    ) -> Stmt {
        let mut cases: Vec<SwitchCase> = vec![];
        for field in &descriptor.field {
            let mut read_expr = self.deserialize_field_expr(ctx, field, accessor, false);
            if field.is_bytes() && ctx.options.with_sendable {
                read_expr = crate::call_expr!(
                        crate::member_expr_bare!(crate::member_expr!("collections", "Uint8Array"), "from"), 
                        vec![
                            crate::expr_or_spread!(read_expr)]
                    )
            }
            let read_stmt = if field.is_map(ctx) {
                crate::expr_stmt!(read_expr)
            } else if field.is_message() && !field.is_repeated() {
                crate::expr_stmt!(read_expr)
            } else if field.is_packable() {
                let field_expr = self.deserialize_field_expr(ctx, field, accessor, true);
                crate::if_stmt!(
                    crate::call_expr!(crate::member_expr!("br", "isDelimited")),
                    crate::expr_stmt!(crate::call_expr!(
                        crate::member_expr_bare!(
                            crate::member_expr!("this", format!("{}?", field.name())),
                            "push"
                        ),
                        vec![crate::expr_or_spread!(field_expr, true)]
                    )),
                    crate::expr_stmt!(crate::call_expr!(
                        crate::member_expr_bare!(
                            crate::member_expr!("this", format!("{}?", field.name())),
                            "push"
                        ),
                        vec![crate::expr_or_spread!(
                            self.deserialize_field_expr(ctx, field, accessor, false)
                        )]
                    ))
                )
            } else if field.is_repeated() && !field.is_packed(ctx) {
                crate::expr_stmt!(crate::call_expr!(
                    crate::member_expr_bare!(crate::member_expr!("this", format!("{}?", field.name())), "push"),
                    vec![crate::expr_or_spread!(read_expr)]
                ))
            } else {
                crate::expr_stmt!(crate::assign_expr!(
                    PatOrExpr::Expr(Box::new(accessor(field))),
                    read_expr
                ))
            };

            let mut stmts = vec![
                read_stmt,
                Stmt::Break(BreakStmt {
                    label: None,
                    span: DUMMY_SP,
                }),
            ];
            if field.is_message() && !field.is_repeated() {
                stmts.insert(
                    0,
                    crate::expr_stmt!(
                        self.deserialize_message_field_preread_expr(ctx, field, accessor)
                    ),
                )
            }

            cases.push(SwitchCase {
                span: DUMMY_SP,
                test: Some(Box::new(crate::lit_num!(field.number() as f64).into())),
                cons: stmts,
            })
        }
        // illegal zero case
        cases.push(SwitchCase {
            span: DUMMY_SP,
            test: Some(Box::new(crate::lit_num!(0.0).into())),
            cons: vec![Stmt::Throw(ThrowStmt {
                span: DUMMY_SP,
                arg: Box::new(crate::new_expr!(
                    quote_ident!("Error").into(),
                    vec![crate::expr_or_spread!(
                        crate::lit_str!("illegal zero tag.").into()
                    )]
                )),
            })],
        });

        // unknown fields

        cases.push(SwitchCase {
            span: DUMMY_SP,
            test: None,
            cons: vec![crate::expr_stmt!(crate::call_expr!(crate::member_expr!(
                "br",
                "skipField"
            )))]
        });

        let switch_stmt = Stmt::Switch(SwitchStmt {
            span: DUMMY_SP,
            discriminant: Box::new(crate::call_expr!(crate::member_expr!(
                "br",
                "getFieldNumber"
            ))),
            cases,
        });

        let while_stmt_test_expr = crate::bin_expr!(
            crate::call_expr!(crate::member_expr!("br", "nextField")),
            crate::unary_expr!(crate::call_expr!(crate::member_expr!("br", "isEndGroup"))),
            BinaryOp::LogicalAnd
        );
        Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(while_stmt_test_expr),
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                stmts: vec![switch_stmt],
            })),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::Syntax,
        descriptor::{
            field_descriptor_proto::{Label, Type},
            DescriptorProto, FieldDescriptorProto, FieldOptions,
        },
        emit::emit,
        options::Options,
    };
    use protobuf::MessageField;
    use swc_ecma_ast::ModuleItem;

    fn render_repeated_field(field_type: Type, syntax: Syntax, packed: Option<bool>) -> String {
        let mut field = FieldDescriptorProto::new();
        field.set_name("values".to_string());
        field.set_number(1);
        field.set_label(Label::LABEL_REPEATED);
        field.set_type(field_type);

        if let Some(packed) = packed {
            let mut options = FieldOptions::new();
            options.set_packed(packed);
            field.options = MessageField::some(options);
        }

        let mut descriptor = DescriptorProto::new();
        descriptor.field.push(field);

        let options = Options::parse("with_sendable=false");
        let mut ctx = Context::new(&options, &syntax);
        let stmt = GooglePBRuntime::new().deserialize_stmt(
            &mut ctx,
            &descriptor,
            field::this_field_member,
            true,
        );

        emit(vec![ModuleItem::Stmt(stmt)])
    }

    #[test]
    fn repeated_double_uses_observed_wire_type_for_all_packing_defaults() {
        for generated in [
            render_repeated_field(Type::TYPE_DOUBLE, Syntax::Proto2, None),
            render_repeated_field(Type::TYPE_DOUBLE, Syntax::Proto2, Some(true)),
            render_repeated_field(Type::TYPE_DOUBLE, Syntax::Proto3, None),
            render_repeated_field(Type::TYPE_DOUBLE, Syntax::Proto3, Some(false)),
        ] {
            assert!(
                generated.contains(
                    "if (br.isDelimited()) this.values?.push(...(br.readPackedDouble() as number[]))"
                ),
                "generated packed branch did not append packed values:\n{generated}"
            );
            assert!(
                generated.contains("else this.values?.push(br.readDouble())"),
                "generated unpacked branch did not read a scalar value:\n{generated}"
            );
        }
    }

    #[test]
    fn packed_bigint_and_bool_values_are_converted_before_appending() {
        let bigint = render_repeated_field(Type::TYPE_INT64, Syntax::Proto3, None);
        assert!(
            bigint.contains(
                "this.values?.push(...(br.readPackedInt64String() as string[]).map((r: string)=>BigInt(r)))"
            ),
            "generated packed bigint branch did not convert every value:\n{bigint}"
        );
        assert!(
            bigint.contains("else this.values?.push(BigInt(br.readInt64String()))"),
            "generated unpacked bigint branch did not convert the scalar value:\n{bigint}"
        );

        let boolean = render_repeated_field(Type::TYPE_BOOL, Syntax::Proto3, None);
        assert!(
            boolean.contains(
                "this.values?.push(...(br.readPackedInt64() as number[]).map((r: number)=>r !== 0))"
            ),
            "generated packed bool branch did not convert every value:\n{boolean}"
        );
        assert!(
            boolean.contains("else this.values?.push(br.readInt64() !== 0)"),
            "generated unpacked bool branch did not convert the scalar value:\n{boolean}"
        );
    }

    #[test]
    fn packed_reader_results_are_typed_as_arrays_before_spreading() {
        let number = render_repeated_field(Type::TYPE_DOUBLE, Syntax::Proto3, None);
        assert!(
            number.contains("this.values?.push(...(br.readPackedDouble() as number[]))"),
            "generated packed number branch did not type the reader result as an array:\n{number}"
        );

        let bigint = render_repeated_field(Type::TYPE_INT64, Syntax::Proto3, None);
        assert!(
            bigint.contains(
                "this.values?.push(...(br.readPackedInt64String() as string[]).map((r: string)=>BigInt(r)))"
            ),
            "generated packed bigint branch did not type the reader result as an array:\n{bigint}"
        );

        let boolean = render_repeated_field(Type::TYPE_BOOL, Syntax::Proto3, None);
        assert!(
            boolean.contains(
                "this.values?.push(...(br.readPackedInt64() as number[]).map((r: number)=>r !== 0))"
            ),
            "generated packed bool branch did not type the reader result as an array:\n{boolean}"
        );
    }

    #[test]
    fn repeated_uint32_normalizes_only_unpacked_values() {
        let generated = render_repeated_field(Type::TYPE_UINT32, Syntax::Proto3, None);
        assert!(
            generated.contains("this.values?.push(...(br.readPackedUint32() as number[]))"),
            "generated packed uint32 branch did not append the packed array:\n{generated}"
        );
        assert!(
            generated.contains("else this.values?.push(br.readUint32() >>> 0)"),
            "generated unpacked uint32 branch did not normalize the scalar value:\n{generated}"
        );
        assert!(
            !generated.contains("readPackedUint32() >>> 0"),
            "generated packed uint32 branch normalized the array as a scalar:\n{generated}"
        );
    }
}
