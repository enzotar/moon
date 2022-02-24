import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class JsonExtract extends HookConsumerWidget {
  JsonExtract({Key? key, required this.node, required this.selected})
      : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final bool selected;
  final controller = useTextEditingController();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
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
                                height: 125,
                                decoration: BoxDecoration(
                                  color: Color(0xFFEEEEEE),
                                  border: Border.all(
                                    color: Color(0xFF258ED5),
                                  ),
                                ),
                                child: Padding(
                                  padding: EdgeInsetsDirectional.fromSTEB(
                                      10, 5, 5, 10),
                                  child: Column(
                                    mainAxisSize: MainAxisSize.max,
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    crossAxisAlignment:
                                        CrossAxisAlignment.start,
                                    children: [
                                      Padding(
                                        padding: EdgeInsetsDirectional.fromSTEB(
                                            5, 0, 0, 0),
                                        child: Text(
                                          'type value',
                                          // style: FlutterFlowTheme
                                          //     .bodyText1
                                          //     .override(
                                          //   fontFamily: 'Poppins',
                                          //   fontWeight: FontWeight.w600,
                                          // ),
                                        ),
                                      ),
                                      Padding(
                                        padding: EdgeInsetsDirectional.fromSTEB(
                                            5, 0, 0, 0),
                                        child: TextFormField(
                                          controller: controller,
                                          obscureText: false,
                                          decoration: InputDecoration(
                                            hintText: '[Some hint text...]',
                                            // hintStyle: FlutterFlowTheme
                                            //     .bodyText1,
                                            enabledBorder: UnderlineInputBorder(
                                              borderSide: BorderSide(
                                                color: Color(0x00000000),
                                                width: 1,
                                              ),
                                              borderRadius:
                                                  const BorderRadius.only(
                                                topLeft: Radius.circular(4.0),
                                                topRight: Radius.circular(4.0),
                                              ),
                                            ),
                                            focusedBorder: UnderlineInputBorder(
                                              borderSide: BorderSide(
                                                color: Color(0x00000000),
                                                width: 1,
                                              ),
                                              borderRadius:
                                                  const BorderRadius.only(
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
                        children: [
                          Container(
                            width: 50,
                            height: 50,
                            decoration: BoxDecoration(
                              color: Color(0xFFEEEEEE),
                            ),
                            child: Column(
                              mainAxisSize: MainAxisSize.max,
                              children: [
                                Align(
                                  alignment: AlignmentDirectional(1, 0),
                                  child: Radio(
                                    value: [true],
                                    groupValue: [true, false],
                                    onChanged: (value) {
                                      // setState(() =>
                                      //     radioButtonValue2 = value);
                                    },
                                  ),
                                ),
                                Text(
                                  'output',
                                  // style: FlutterFlowTheme.bodyText1
                                  //     .override(
                                  //   fontFamily: 'Poppins',
                                  //   fontSize: 10,
                                  //   fontWeight: FontWeight.normal,
                                  // ),
                                ),
                              ],
                            ),
                          ),
                        ],
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
