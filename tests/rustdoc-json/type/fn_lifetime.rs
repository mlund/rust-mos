// ignore-tidy-linelength

// @has "$.index[*][?(@.name=='GenericFn')].inner.typedef"

// @ismany "$.index[*][?(@.name=='GenericFn')].inner.typedef.generics.params[*].name" \"\'a\"
// @has    "$.index[*][?(@.name=='GenericFn')].inner.typedef.generics.params[*].kind.lifetime"
// @count  "$.index[*][?(@.name=='GenericFn')].inner.typedef.generics.params[*].kind.lifetime.outlives[*]" 0
// @count  "$.index[*][?(@.name=='GenericFn')].inner.typedef.generics.where_predicates[*]" 0
// @count  "$.index[*][?(@.name=='GenericFn')].inner.typedef.type.function_pointer.generic_params[*]" 0
// @count  "$.index[*][?(@.name=='GenericFn')].inner.typedef.type.function_pointer.decl.inputs[*]" 1
// @is     "$.index[*][?(@.name=='GenericFn')].inner.typedef.type.function_pointer.decl.inputs[*][1].borrowed_ref.lifetime" \"\'a\"
// @is     "$.index[*][?(@.name=='GenericFn')].inner.typedef.type.function_pointer.decl.output.borrowed_ref.lifetime" \"\'a\"

pub type GenericFn<'a> = fn(&'a i32) -> &'a i32;

// @has    "$.index[*][?(@.name=='ForAll')].inner.typedef"
// @count "$.index[*][?(@.name=='ForAll')].inner.typedef.generics.params[*]" 0
// @count "$.index[*][?(@.name=='ForAll')].inner.typedef.generics.where_predicates[*]" 0
// @count "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.generic_params[*]" 1
// @is    "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.generic_params[*].name" \"\'a\"
// @has   "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.generic_params[*].kind.lifetime"
// @count "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.generic_params[*].kind.lifetime.outlives[*]" 0
// @count "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.decl.inputs[*]" 1
// @is    "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.decl.inputs[*][1].borrowed_ref.lifetime" \"\'a\"
// @is    "$.index[*][?(@.name=='ForAll')].inner.typedef.type.function_pointer.decl.output.borrowed_ref.lifetime" \"\'a\"
pub type ForAll = for<'a> fn(&'a i32) -> &'a i32;
