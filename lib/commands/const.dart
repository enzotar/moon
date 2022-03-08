import 'dart:convert';
import 'dart:ui';

import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:recase/recase.dart';
import 'package:rheetah/commands/const_subblocks/json_text_field.dart';
import 'package:rheetah/commands/const_subblocks/nft_metadata_form.dart';
import 'package:rheetah/providers/store_provider.dart';
import 'package:rheetah/serialization/input_mapping.dart';
import 'package:rheetah/widgets/block.dart';
import 'package:tuple/tuple.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:file_picker/file_picker.dart';

// type should match the Value in Sunshine Solana
String createJson<T>(T value, String nodeId, [String? type]) {
  Map<String, T> inputProperties;
  String outer;
  Map<String, dynamic> outerMap;

  if (type != null) {
    inputProperties = {type: value};
    var outerMap = {"Const": inputProperties};
    outer = JsonMapper.serialize(InputProperties(outerMap));
    var combined = {"nodeId": nodeId, "text": outer};
    print(combined);

    return JsonMapper.serialize(InputProperties(combined));
  } else {
    // final inputProperties = {
    //   "nodeId": treeNode.node.key,
    //   "text": text,
    // };
    // print(inputProperties);
    // String output = JsonMapper.serialize(InputProperties(inputProperties));

    // outerMap = value as Map<String, dynamic>;
    var outerMap = {"Const": value};
    outer = JsonMapper.serialize(InputProperties(outerMap));

    var combined = {"nodeId": nodeId, "text": outer};
    print(combined);

    return JsonMapper.serialize(InputProperties(combined));
    // JsonMapper.serialize(InputProperties(value as Map<String, dynamic>));
  }
  // var input = JsonMapper.serialize(InputProperties(inputProperties));
}

class Const extends HookConsumerWidget {
  Const({
    Key? key,
    required this.treeNode,
  }) : super(key: key);

  final TreeNode treeNode;
  // final key = useMemoized(valueBuilder)//

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final focusNode = useFocusNode();
    final dropDownFocusNode = useFocusNode();
    // final provider = ref.watch(changesController);
    final store = ref.read(storeRepoProvider).store;
    ref.watch(nodeController);

    ValueNotifier<String> dropDownValue =
        treeNode.node.value.additionalData == ""
            ? useState("string")
            : useState(treeNode.node.value.additionalData);
    // ValueNotifier<int> height = useState(treeNode.node.value.height);
    // ValueNotifier<int> width = useState(treeNode.node.value.width);

    // final inputProperties = {
    //           "nodeId": treeNode.node.key,
    //           "text": controller.value.text,
    //         };

    final valueList = ref.read(dropDownValues(treeNode));
    // final Map<String, Tuple4<String, int, int, Function>> valueList = {
    //   "JSON": Tuple4("json", 400, 500, () {}),
    //   "Boolean, True": Tuple4("bool_true", 300, 110, () {
    //     final value = createJson(
    //       true,
    //       treeNode.node.key,
    //       "Bool",
    //     );
    //     print(value);
    //     store.msgSendJson(value);
    //   }),
    //   "Boolean, False": Tuple4("bool_false", 300, 110, () {
    //     final value = createJson(
    //       false,
    //       treeNode.node.key,
    //       "Bool",
    //     );
    //     store.msgSendJson(value);
    //   }),
    //   "String": Tuple4("string", 300, 300, () {}),
    //   "NFT Metadata": Tuple4("nft", 300, 600, () {}),
    //   "Seed Phrase": Tuple4("seed", 400, 220, () {}),
    //   "Number, i64": Tuple4("i64", 300, 175, () {}),
    //   "Number, u8": Tuple4("u8", 300, 175, () {}),
    //   "Number, u16": Tuple4("u16", 300, 175, () {}),
    //   "Number, u64": Tuple4("u64", 300, 175, () {}),
    //   "Number, f32": Tuple4("f32", 300, 175, () {}),
    //   "Number, f64": Tuple4("f64", 300, 175, () {}),
    // };
    //   ref.read(storeRepoProvider).store.msgSendJson(arg0)
    //

    useEffect(() {
      // print("rebuilding const");
      dropDownValue.addListener(() {
        final MapEntry<String, Tuple4<String, int, int, Function>> choice =
            valueList.entries.firstWhere(
                (element) => element.value.item1 == dropDownValue.value);
        // call setJson
        //update dimensions
        store
            .msgUpdateDimensions(
          treeNode.node.key,
          dropDownValue.value,
          choice.value.item2,
          choice.value.item3,
        )
            .then((value) {
          choice.value.item4.call();
          dropDownFocusNode.unfocus();

          // set dimensions on repo

          // height.value = jsonDecode(value.data!)["height"];
          // width.value = jsonDecode(value.data!)["width"];

          // print(height);
          // print(width);
        });
      });
      focusNode.addListener(() {
        // if (focusNode.hasFocus) {
        //   ref.watch(focusRejectController.notifier).set([focusNode.rect]);
        // }

        if (focusNode.hasFocus) {
          ref.watch(focusRejectController.notifier).set([focusNode.rect]);
        } else {
          ref.watch(focusRejectController.notifier).set([]);
        }

        print(focusNode.rect);
        print("Has focus:${focusNode.hasFocus}");
        // if (!focusNode.hasFocus) {
        //   print("lost focus ");
        //   ref.watch(focusRejectController.notifier).set([]);
        // }
        //   ref.read(storeRepoProvider).store.msgStartInput("start");
        // }
        // if (!focusNode.hasFocus) {
        //   ref.read(storeRepoProvider).store.msgStopInput("start");
        // }
      });

      return; // You need this return if you have missing_return lint
    }, [focusNode]);

    List<DropdownMenuItem<String>> dropDownList = valueList
        .map<String, DropdownMenuItem<String>>(
          (k, v) {
            return MapEntry(
              k,
              DropdownMenuItem(child: Text(k), value: v.item1),
            );
          },
        )
        .values
        .toList();

    return Container(
      width: treeNode.node.value.width - 120,
      child: Padding(
        padding: EdgeInsetsDirectional.fromSTEB(10, 10, 10, 10),
        child: Container(
          // decoration: BoxDecoration(
          //   color: Color(0xFFEEEEEE),
          //   border: Border.all(
          //     color: Color(0xFF258ED5),
          //   ),
          // // ),
          // width: treeNode.node.value.width - 120,
          // height: treeNode.node.value.height - 120,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                width: treeNode.node.value.width - 120,
                height: 50,
                child: DropdownButton(
                  focusNode: dropDownFocusNode,
                  isExpanded: true,
                  items: dropDownList,
                  onChanged: (value) {
                    dropDownValue.value = value.toString();
                    focusNode.requestFocus(); // FIXME
                  },
                  value: dropDownValue.value,
                ),
              ),
              addTextField(dropDownValue.value, treeNode, focusNode).call(),
            ],
            // ),
          ),
        ),
      ),
    );
  }
}

/// Text Field Router
Function addTextField(
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

final formKey = GlobalKey<FormBuilderState>();

class BoolField extends HookConsumerWidget {
  BoolField({
    Key? key,
    required this.treeNode,
    required this.focusNode,
    required this.boolValue,
  }) : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;
  bool boolValue;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final focusNode = useFocusNode();
    // final scrollController = useScrollController();
    // var controller = useTextEditingController(text: treeNode.node.value.text);
    // final store = ref.read(storeRepoProvider).store;

    return Container(
        // child: ToggleButtons(children: children, isSelected: isSelected),
        );
  }
}

class StringTextField extends HookConsumerWidget {
  StringTextField({Key? key, required this.treeNode, required this.focusNode})
      : super(key: key);

  final TreeNode treeNode;
  final FocusNode focusNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final focusNode = useFocusNode();
    final scrollController = useScrollController();
    var controller = useTextEditingController(
        text: treeNode.node.value.text != ""
            ? jsonDecode(treeNode.node.value.text)["Const"]["String"]
            : treeNode.node.value.text);
    final store = ref.read(storeRepoProvider).store;

    saveToDb() {
      final text = controller.value.text.trimRight();

      final inputEvent = createJson(
        text,
        treeNode.node.key,
        "String",
      );
      store.msgSendJson(inputEvent);
    }

    focusNode.addListener((() {
      print(focusNode.hasFocus);
      if (!focusNode.hasFocus) {
        saveToDb();
      }
    }));

    return Expanded(
      child: Container(
        child: SingleChildScrollView(
          child: Padding(
            padding: const EdgeInsets.fromLTRB(0, 0, 0, 0),
            child: Container(
              decoration: BoxDecoration(
                color: Color(0xFFEEEEEE),
                border: Border.all(
                  color: Color(0xFF258ED5),
                ),
              ),
              width: treeNode.node.value.width - 120,
              height: treeNode.node.value.height - 120,
              child: TextField(
                dragStartBehavior: DragStartBehavior.start,
                expands: true,
                onTap: () {
                  // focusNode.requestFocus();
                },
                focusNode: focusNode,
                minLines: null,
                maxLines: null,
                keyboardType: TextInputType.multiline,
                textInputAction: TextInputAction.done,
                onEditingComplete: () {},
                scrollController: scrollController,

                onChanged: (value) {
                  // final inputProperties = {
                  //   "node_id": "dummy",
                  //   "value": value,
                  // };
                  // String inputEvent =
                  //     JsonMapper.serialize(
                  //         InputProperties(
                  //             inputProperties));
                  // store.store.msgSetText(inputEvent);
                },
                onSubmitted: (_) {
                  print(controller.value.text);
                  saveToDb();
                },
                controller: controller,
                obscureText: false,
                decoration: InputDecoration(
                  // filled: true,
                  border: OutlineInputBorder(),
                  // focusColor: Colors.amber,
                  // hintText: '[Some hint text...]',
                  // hintStyle: FlutterFlowTheme
                  //     .bodyText1,
                  enabledBorder: UnderlineInputBorder(
                    borderSide: BorderSide(
                      color: Color(0x00000000),
                      width: 1,
                    ),
                    borderRadius: const BorderRadius.only(
                      topLeft: Radius.circular(2.0),
                      topRight: Radius.circular(2.0),
                    ),
                  ),
                  disabledBorder: InputBorder.none,
                  focusedBorder: UnderlineInputBorder(
                    borderSide: BorderSide(
                      color: Color(0x00000000),
                      width: 2,
                    ),
                    borderRadius: const BorderRadius.only(
                      topLeft: Radius.circular(2.0),
                      topRight: Radius.circular(2.0),
                    ),
                  ),
                ),
                // style: FlutterFlowTheme
                //     .bodyText1,
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class SeedTextField extends HookConsumerWidget {
  SeedTextField({Key? key, required this.treeNode}) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final focusNode = useFocusNode();
    // final scrollController = useScrollController();
    print(treeNode.node.value.text);
    var controller = useTextEditingController(
        text: treeNode.node.value.text != ""
            ? jsonDecode(treeNode.node.value.text)["Const"]["String"]
            : treeNode.node.value.text);
    final store = ref.read(storeRepoProvider).store;
    final _error = useState("");

    return Expanded(
      child: Container(
        width: treeNode.node.value.width - 120,
        height: 100,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(0, 0, 0, 0),
          child: Column(
            children: [
              Container(
                decoration: BoxDecoration(
                  color: Color(0xFFEEEEEE),
                  border: Border.all(
                    color: Color(0xFF258ED5),
                  ),
                ),
                child: TextField(
                  dragStartBehavior: DragStartBehavior.start,
                  // expands: true,
                  onTap: () {
                    // focusNode.requestFocus();
                  },
                  focusNode: focusNode,
                  minLines: null,
                  maxLines: 2,
                  keyboardType: TextInputType.multiline,
                  textInputAction: TextInputAction.done,
                  onEditingComplete: () {},
                  onChanged: (value) {
                    _error.value = "";
                    final String s = controller.value.text;
                    final RegExp regExp = new RegExp(r"[\w-._]+");
                    final Iterable matches = regExp.allMatches(s);
                    final int _count = matches.length;
                    if (_count != 12) _error.value = "should have 12 words";
                    print(_count);
                  },
                  // scrollController: scrollController,

                  // onChanged: (value) {
                  //   final inputProperties = {
                  //     "node_id": "dummy",
                  //     "value": value,
                  //   };
                  //   String inputEvent =
                  //       JsonMapper.serialize(
                  //           InputProperties(
                  //               inputProperties));
                  //   store.store.msgSetText(inputEvent);
                  // },
                  onSubmitted: (_) {
                    print(controller.value.text);
                    final text = controller.value.text.trimRight();
                    // final inputProperties = {
                    //   "nodeId": treeNode.node.key,
                    //   "text": controller.value.text,
                    // };
                    // String inputEvent =
                    //     JsonMapper.serialize(InputProperties(inputProperties));
                    final inputEvent = createJson(
                      text,
                      treeNode.node.key,
                      "String",
                    );
                    store.msgSendJson(inputEvent);
                  },
                  controller: controller,
                  obscureText: false,
                  decoration: InputDecoration(
                    focusedErrorBorder: InputBorder.none,
                    errorBorder: InputBorder.none,
                    errorText: _error.value,
                    // filled: true,
                    border: OutlineInputBorder(),
                    // focusColor: Colors.amber,
                    // hintText: '[Some hint text...]',
                    // hintStyle: FlutterFlowTheme
                    //     .bodyText1,
                    enabledBorder: UnderlineInputBorder(
                      borderSide: BorderSide(
                        color: Color(0x00000000),
                        width: 1,
                      ),
                      borderRadius: const BorderRadius.only(
                        topLeft: Radius.circular(2.0),
                        topRight: Radius.circular(2.0),
                      ),
                    ),
                    disabledBorder: InputBorder.none,
                    focusedBorder: UnderlineInputBorder(
                      borderSide: BorderSide(
                        color: Color(0x00000000),
                        width: 2,
                      ),
                      borderRadius: const BorderRadius.only(
                        topLeft: Radius.circular(2.0),
                        topRight: Radius.circular(2.0),
                      ),
                    ),
                  ),
                  // style: FlutterFlowTheme
                  //     .bodyText1,
                ),
              ),
              Container(
                height: 20,
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    IconButton(
                      onPressed: () {},
                      icon: Icon(Icons.refresh_outlined),
                    ),
                    Text(
                      "Generate new seed",
                      style: TextStyle(fontSize: 12),
                      textAlign: TextAlign.center,
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class NumberTextField extends HookConsumerWidget {
  NumberTextField({
    Key? key,
    required this.treeNode,
    required this.numberType,
    required this.numberIs,
  }) : super(key: key);

  final TreeNode treeNode;
  final numberType;
  final numberIs;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final focusNode = useFocusNode();
    final scrollController = useScrollController();
    var controller = useTextEditingController(text: treeNode.node.value.text);
    final store = ref.read(storeRepoProvider).store;

    // ReCase rc = ReCase(numberType);
    ValueNotifier<String> _error = useState("");

    saveToDb() {
      final inputEvent = createJson(
        numberIs == int
            ? int.parse(controller.value.text)
            : double.parse(controller.value.text),
        treeNode.node.key,
        numberType.toString().toUpperCase(),
      );
      store.msgSendJson(inputEvent);
    }

    focusNode.addListener((() {
      print(focusNode.hasFocus);
      if (!focusNode.hasFocus) {
        saveToDb();
      }
    }));

    return Container(
      child: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(0, 0, 0, 0),
          child: Container(
            decoration: BoxDecoration(
              color: Color(0xFFEEEEEE),
              border: Border.all(
                color: Color(0xFF258ED5),
              ),
            ),
            width: treeNode.node.value.width - 120,
            height: treeNode.node.value.height - 120,
            child: TextField(
              dragStartBehavior: DragStartBehavior.start,
              // expands: true,

              onTap: () {
                // focusNode.requestFocus();
              },
              focusNode: focusNode,
              minLines: null,
              maxLines: 1,
              keyboardType: TextInputType.number,
              textInputAction: TextInputAction.done,
              onEditingComplete: () {},
              onChanged: (value) {
                // final text = _controller.value.text;
                // Note: you can do your own custom validation here
                // Move this logic this outside the widget for more testable code
                _error.value = "";

                if (numberIs == int) {
                  final number = int.tryParse(value);
                  if (number == null) _error.value = 'Must be a integer';
                }
                if (numberIs == double) {
                  final float = double.tryParse(value);
                  if (float == null) _error.value = 'Must be a float';
                }
                if (value.isEmpty) {
                  _error.value = "";
                }

                // return null if the text is valid
                // return null;
              },
              scrollController: scrollController,

              // onChanged: (value) {
              //   final inputProperties = {
              //     "node_id": "dummy",
              //     "value": value,
              //   };
              //   String inputEvent =
              //       JsonMapper.serialize(
              //           InputProperties(
              //               inputProperties));
              //   store.store.msgSetText(inputEvent);
              // },
              onSubmitted: (_) {
                saveToDb();
              },
              controller: controller,
              obscureText: false,
              decoration: InputDecoration(
                focusedErrorBorder: InputBorder.none,
                errorBorder: InputBorder.none,
                errorText: _error.value,
                // filled: true,
                border: OutlineInputBorder(),
                // focusColor: Colors.amber,
                // hintText: '[Some hint text...]',
                // hintStyle: FlutterFlowTheme
                //     .bodyText1,
                enabledBorder: UnderlineInputBorder(
                  borderSide: BorderSide(
                    color: Color(0x00000000),
                    width: 1,
                  ),
                  borderRadius: const BorderRadius.only(
                    topLeft: Radius.circular(2.0),
                    topRight: Radius.circular(2.0),
                  ),
                ),
                disabledBorder: InputBorder.none,
                focusedBorder: UnderlineInputBorder(
                  borderSide: BorderSide(
                    color: Color(0x00000000),
                    width: 2,
                  ),
                  borderRadius: const BorderRadius.only(
                    topLeft: Radius.circular(2.0),
                    topRight: Radius.circular(2.0),
                  ),
                ),
              ),
              // style: FlutterFlowTheme
              //     .bodyText1,
            ),
          ),
        ),
      ),
    );
  }
}
