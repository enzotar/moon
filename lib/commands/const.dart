import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';
import 'package:moon/serialization/input_mapping.dart';

class Const extends HookConsumerWidget {
  Const({
    Key? key,
    required this.node,
    required this.selected,
    required this.inputs,
    required this.outputs,
  }) : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final bool selected;
  final focusNode = useFocusNode();
  final List<Widget> inputs;
  final List<Widget> outputs;
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var controller = useTextEditingController(text: node.value.text);

    final store = ref.watch(storeRepoProvider);
    useEffect(() {
      focusNode.addListener(() {
        print("Has focus:${focusNode.hasFocus}");
      });
      return; // You need this return if you have missing_return lint
    }, [focusNode]);

    return Positioned(
        // height: node.value.height.toDouble(),
        // width: node.value.width.toDouble(),
        // left: node.value.x.toDouble(),
        // top: node.value.y.toDouble(),
        child: Card(
      color: Color(0xFFF5F5F5),
      child: Container(
        decoration: BoxDecoration(
          color: Color(0xFFEEEEEE),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: Row(
                mainAxisSize: MainAxisSize.max,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Expanded(
                    child: Padding(
                      padding: EdgeInsetsDirectional.fromSTEB(10, 5, 5, 10),
                      child: Column(
                        mainAxisSize: MainAxisSize.max,
                        mainAxisAlignment: MainAxisAlignment.start,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Container(
                            width: 100,
                            height: 50,
                            decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(0),
                            ),
                            child: Padding(
                              padding: EdgeInsetsDirectional.fromSTEB(
                                  10, 10, 10, 10),
                              child: Text(
                                'Constant Input',
                                style: TextStyle(
                                    backgroundColor: Color(0xFFEEEEEE),
                                    fontSize: 20.00,
                                    fontWeight: FontWeight.bold),
                                // style: FlutterFlowTheme.title1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   fontSize: 18,
                                // ),
                              ),
                            ),
                          ),
                          Container(
                            width: 200,
                            height: 200,
                            decoration: BoxDecoration(
                              color: Color(0xFFEEEEEE),
                              border: Border.all(
                                color: Color(0xFF258ED5),
                              ),
                            ),
                            child: Padding(
                              padding:
                                  EdgeInsetsDirectional.fromSTEB(10, 5, 5, 10),
                              child: Column(
                                mainAxisSize: MainAxisSize.max,
                                mainAxisAlignment: MainAxisAlignment.start,
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Padding(
                                    padding: EdgeInsetsDirectional.fromSTEB(
                                        5, 0, 0, 0),
                                    child: TextField(
                                      focusNode: focusNode,
                                      minLines: 3,
                                      maxLines: null,
                                      keyboardType: TextInputType.multiline,
                                      textInputAction: TextInputAction.done,

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
                                        final inputProperties = {
                                          "nodeId": node.key,
                                          "text": controller.value.text,
                                        };
                                        String inputEvent =
                                            JsonMapper.serialize(
                                                InputProperties(
                                                    inputProperties));

                                        store.store.msgSendJson(inputEvent);
                                      },
                                      controller: controller,
                                      obscureText: false,
                                      decoration: InputDecoration(
                                        // hintText: '[Some hint text...]',
                                        // hintStyle: FlutterFlowTheme
                                        //     .bodyText1,
                                        enabledBorder: UnderlineInputBorder(
                                          borderSide: BorderSide(
                                            color: Color(0x00000000),
                                            width: 1,
                                          ),
                                          borderRadius: const BorderRadius.only(
                                            topLeft: Radius.circular(4.0),
                                            topRight: Radius.circular(4.0),
                                          ),
                                        ),
                                        focusedBorder: UnderlineInputBorder(
                                          borderSide: BorderSide(
                                            color: Color(0x00000000),
                                            width: 1,
                                          ),
                                          borderRadius: const BorderRadius.only(
                                            topLeft: Radius.circular(4.0),
                                            topRight: Radius.circular(4.0),
                                          ),
                                        ),
                                      ),
                                      // style: FlutterFlowTheme
                                      //     .bodyText1,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: outputs,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    ));
  }
}
