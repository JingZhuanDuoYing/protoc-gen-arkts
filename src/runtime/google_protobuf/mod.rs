 /**
  * Copyright 2024 ByteDance and/or its affiliates
  *
  * Original Files：protoc-gen-ts (https://github.com/thesayyn/protoc-gen-ts)
  * Copyright (c) 2024 Sahin Yort
  * SPDX-License-Identifier: MIT 
 */

use crate::common::field;
use crate::{
    context::Context,
    descriptor::{self, field_descriptor_proto::Type, FieldDescriptorProto},
    runtime::Runtime,
};
use swc_ecma_ast::Stmt;

#[derive(Clone)]
pub struct GooglePBRuntime {}

impl Runtime for GooglePBRuntime {
    fn from_binary(
        &self,
        ctx: &mut Context,
        descriptor: &descriptor::DescriptorProto,
    ) -> Vec<Stmt> {
        self.deserialize_setup_inner(ctx, descriptor, true)
    }

    fn to_binary(
        &self,
        ctx: &mut Context,
        descriptor: &descriptor::DescriptorProto,
    ) -> Vec<Stmt> {
        self.serialize_setup_inner(ctx, descriptor, field::this_field_member, false, true)
    }
}

impl GooglePBRuntime {
    pub fn new() -> Self {
        GooglePBRuntime {}
    }

    fn rw_function_name(
        &self,
        rw: &str,
        ctx: &mut Context,
        field: &FieldDescriptorProto,
    ) -> String {
        self.rw_function_name_for_encoding(rw, field, field.is_packed(ctx))
    }

    fn rw_function_name_for_encoding(
        &self,
        rw: &str,
        field: &FieldDescriptorProto,
        packed: bool,
    ) -> String {
        let mut placeholder = format!("{}", rw);
        if packed {
            placeholder = format!("{}Packed", rw);
        }
        match field.type_() {
            Type::TYPE_STRING => "_placeholder_String",
            Type::TYPE_BOOL => "_placeholder_Int64",
            Type::TYPE_FLOAT => "_placeholder_Float",
            Type::TYPE_DOUBLE => "_placeholder_Double",
            Type::TYPE_ENUM => "_placeholder_Int32",
            Type::TYPE_BYTES => "_placeholder_Bytes",

            Type::TYPE_INT32 => "_placeholder_Int32",
            Type::TYPE_INT64 => "_placeholder_Int64String",
            Type::TYPE_UINT32 => "_placeholder_Uint32",
            Type::TYPE_UINT64 => "_placeholder_Uint64String",
            Type::TYPE_SINT32 => "_placeholder_Sint32",
            Type::TYPE_SINT64 => "_placeholder_Sint64String",

            Type::TYPE_FIXED32 => "_placeholder_Fixed32",
            Type::TYPE_FIXED64 => "_placeholder_Fixed64String",
            Type::TYPE_SFIXED32 => "_placeholder_Sfixed32",
            Type::TYPE_SFIXED64 => "_placeholder_Sfixed64String",

            Type::TYPE_GROUP => "skipField",
            Type::TYPE_MESSAGE => "skipField",
        }
        .replace("_placeholder_", placeholder.as_str())
    }
}

pub mod deserialize;
pub mod serialize;
pub mod well_known;
