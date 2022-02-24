import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class GenerateKeypair extends HookConsumerWidget {
  GenerateKeypair({Key? key, required this.node, required this.selected})
      : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final bool selected;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Positioned(
        // height: node.value.height.toDouble(),
        // width: node.value.width.toDouble(),
        // left: node.value.x.toDouble(),
        // top: node.value.y.toDouble(),
        child: Card(
      color: Colors.orange.shade200,
      child: Container(
        decoration: BoxDecoration(
          color: Colors.orange.shade200,
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
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: [
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
                        ),
                        child: Column(
                          mainAxisSize: MainAxisSize.max,
                          children: [
                            Align(
                              alignment: AlignmentDirectional(-1, 0),
                              child: Radio(
                                value: [true],
                                groupValue: [true, false],
                                onChanged: (value) {
                                  // setState(() =>
                                  //     radioButtonValue1 = value);
                                },
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'seed',
                              // style: FlutterFlowTheme.bodyText1
                              //     .override(
                              //   fontFamily: 'Poppins',
                              //   fontSize: 10,
                              //   fontWeight: FontWeight.normal,
                            ),
                            // ),
                          ],
                        ),
                      ),
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
                        ),
                        child: Column(
                          mainAxisSize: MainAxisSize.max,
                          children: [
                            Align(
                              alignment: AlignmentDirectional(-1, 0),
                              child: Radio(
                                value: [true],
                                groupValue: [true, false],
                                onChanged: (value) {
                                  // setState(() =>
                                  //     radioButtonValue1 = value);
                                },
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'passphrase',
                              // style: FlutterFlowTheme.bodyText1
                              //     .override(
                              //   fontFamily: 'Poppins',
                              //   fontSize: 10,
                              //   fontWeight: FontWeight.normal,
                            ),
                            // ),
                          ],
                        ),
                      ),
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
                        ),
                        child: Column(
                          mainAxisSize: MainAxisSize.max,
                          children: [
                            Align(
                              alignment: AlignmentDirectional(-1, 0),
                              child: Radio(
                                value: [true],
                                groupValue: [true, false],
                                onChanged: (value) {
                                  // setState(() =>
                                  //     radioButtonValue1 = value);
                                },
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'save',
                              // style: FlutterFlowTheme.bodyText1
                              //     .override(
                              //   fontFamily: 'Poppins',
                              //   fontSize: 10,
                              //   fontWeight: FontWeight.normal,
                            ),
                            // ),
                          ],
                        ),
                      ),
                    ],
                  ),
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
                            height: 100,
                            decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(0),
                            ),
                            child: Padding(
                              padding: EdgeInsetsDirectional.fromSTEB(
                                  10, 10, 10, 10),
                              child: Text(
                                'Generate Keypair',
                                style: TextStyle(
                                    backgroundColor: Colors.orange.shade200,
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
                        ],
                      ),
                    ),
                  ),
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: [
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
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
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'pubkey',
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
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
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
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'keypair',
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
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
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
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'empty',
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
                      Container(
                        width: 75,
                        height: 50,
                        decoration: BoxDecoration(
                          color: Colors.orange.shade200,
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
                                // optionHeight: 35,
                                // textStyle: FlutterFlowTheme
                                //     .bodyText1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   color: Colors.black,
                                // ),
                                // buttonPosition:
                                //     RadioButtonPosition.left,
                                // direction: Axis.vertical,
                                // radioButtonColor: Colors.blue,
                                // inactiveRadioButtonColor:
                                //     Color(0x8A000000),
                                // toggleable: false,
                                // horizontalAlignment:
                                //     WrapAlignment.start,
                                // verticalAlignment:
                                //     WrapCrossAlignment.start,
                              ),
                            ),
                            Text(
                              'node id',
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
