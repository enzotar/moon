import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/commands/const_subblocks/bool_field.dart';
import 'package:moon/commands/const_subblocks/file_picker.dart';
import 'package:moon/commands/const_subblocks/json_text_field.dart';
import 'package:moon/commands/const_subblocks/nft_metadata_form.dart';
import 'package:moon/commands/const_subblocks/numbers_field.dart';
import 'package:moon/commands/const_subblocks/seed_phrase_field.dart';
import 'package:moon/commands/const_subblocks/string_field.dart';
import 'package:moon/providers/store_provider.dart';

import 'package:tuple/tuple.dart';

final dropDownValues =
    Provider.family<Map<String, Tuple4<String, int, int, Function>>, TreeNode>(
        (ref, treeNode) {
  return {
    "JSON": Tuple4("json", 400, 500, () {}),
    "Boolean, True": Tuple4("bool_true", 300, 110, () {}),
    "Boolean, False": Tuple4("bool_false", 300, 110, () {}),
    "String": Tuple4("string", 300, 300, () {}),
    "File Picker": Tuple4("file_picker", 300, 300, () {}),
    "NFT Metadata": Tuple4("nft", 300, 600, () {}),
    "Seed Phrase": Tuple4("seed", 400, 220, () {}),
    "Number, i64": Tuple4("i64", 300, 175, () {}),
    "Number, u8": Tuple4("u8", 300, 175, () {}),
    "Number, u16": Tuple4("u16", 300, 175, () {}),
    "Number, u64": Tuple4("u64", 300, 175, () {}),
    "Number, f32": Tuple4("f32", 300, 175, () {}),
    "Number, f64": Tuple4("f64", 300, 175, () {}),
  };
});

/// Child Field Router
Function buildChildField(
    String? fieldType, TreeNode treeNode, FocusNode focusNode) {
  if (fieldType == null) {
    return (List<dynamic> inputs) => Container();
  } else {
    final widgetStore = <String, Function>{
      "json": () => JsonTextField(treeNode: treeNode),
      "bool_true": () =>
          BoolField(treeNode: treeNode, focusNode: focusNode, boolValue: true),
      "bool_false": () =>
          BoolField(treeNode: treeNode, focusNode: focusNode, boolValue: false),
      "string": () => StringTextField(treeNode: treeNode, focusNode: focusNode),
      "file_picker": () =>
          FilePickerField(treeNode: treeNode, focusNode: focusNode),
      "nft": () => NftMetadataForm(treeNode: treeNode, focusNode: focusNode),
      "seed": () => SeedTextField(treeNode: treeNode),
      "i64": () =>
          NumberTextField(treeNode: treeNode, numberType: "I64", numberIs: int),
      "u8": () =>
          NumberTextField(treeNode: treeNode, numberType: "U8", numberIs: int),
      "u16": () =>
          NumberTextField(treeNode: treeNode, numberType: "U16", numberIs: int),
      "u64": () =>
          NumberTextField(treeNode: treeNode, numberType: "U64", numberIs: int),
      "f32": () => NumberTextField(
          treeNode: treeNode, numberType: "F32", numberIs: double),
      "f64": () => NumberTextField(
          treeNode: treeNode, numberType: "F64", numberIs: double),
    };

    return widgetStore.entries
        .firstWhere((element) => element.key == fieldType)
        .value;
  }
}
