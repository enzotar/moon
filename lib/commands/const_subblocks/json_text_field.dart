import 'dart:convert';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:recase/recase.dart';
import 'package:moon/commands/const.dart';
import 'package:moon/providers/store_provider.dart';

class JsonTextField extends HookConsumerWidget {
  JsonTextField({Key? key, required this.treeNode}) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final focusNode = useFocusNode();
    final scrollController = useScrollController();
    var controller = useTextEditingController(
        text: treeNode.node.value.text == ""
            ? ""
            : jsonEncode(jsonDecode(treeNode.node.value.text)["Const"]));
    final store = ref.read(storeRepoProvider).store;

    ValueNotifier<String> _error = useState("");
    ValueNotifier<bool> decodeSucceeded = useState(false);
    ValueNotifier<Map<String, dynamic>> decodedJson = useState({});

    saveToDb() {
      final text = decodedJson.value;
      final output =
          createJson<Map<String, dynamic>>(text, treeNode.node.key, null);

      print(output);
      store.msgSendJson(output);
    }

    focusNode.addListener((() {
      if (!focusNode.hasFocus) {
        saveToDb();
      }
    }));
    return Expanded(
      child: Container(
        height: treeNode.node.value.height - 120,
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
                  _error.value = "";

                  decodeSucceeded.value = false;
                  try {
                    decodedJson.value =
                        json.decode(value) as Map<String, dynamic>;
                    decodeSucceeded.value = true;
                  } on FormatException catch (e) {
                    print(e);
                    _error.value = "Not a valid JSON";
                  }
                },
                onSubmitted: (_) {
                  if (decodeSucceeded.value == true) saveToDb();
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
      ),
    );
  }
}
