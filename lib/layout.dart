import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/logger.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'event_listener.dart';

class LayoutScreen extends HookConsumerWidget {
  static const routeName = "/Create";

  final TransformationController _transformationController =
      TransformationController();

  double height = 3000;
  double width = 3000;

  // https://api.flutter.dev/flutter/widgets/InteractiveViewer/transformationController.html
  // Animation<Matrix4>? _animationReset;
  // late final AnimationController _controllerReset;

  // void _onAnimateReset() {
  //   _transformationController.value = _animationReset!.value;
  //   if (!_controllerReset.isAnimating) {
  //     _animationReset!.removeListener(_onAnimateReset);
  //     _animationReset = null;
  //     _controllerReset.reset();
  //   }
  // }

  // void _onInteractionStart(ScaleStartDetails details) {
  //   // If the user tries to cause a transformation while the reset animation is
  //   // running, cancel the reset animation.
  //   if (_controllerReset.status == AnimationStatus.forward) {
  //     _animateResetStop();
  //   }
  // }

  // void _animateResetInitialize() {
  //   _controllerReset.reset();
  //   _animationReset = Matrix4Tween(
  //     begin: _transformationController.value,
  //     end: Matrix4.identity(),
  //   ).animate(_controllerReset);
  //   _animationReset!.addListener(_onAnimateReset);
  //   _controllerReset.forward();
  // }

  // // Stop a running reset to home transform animation.
  // void _animateResetStop() {
  //   _controllerReset.stop();
  //   _animationReset?.removeListener(_onAnimateReset);
  //   _animationReset = null;
  //   _controllerReset.reset();
  // }

  @override
  Widget build(BuildContext buildContext, WidgetRef ref) {
    var _last = useState("");
    final store = ref.watch(storeRepoProvider);

    final List<rid.GraphEntry> graphList = rid.Store.instance.view.graphList;

    List<DropdownMenuItem<String>> dropDownList = graphList.map(
      (e) {
        return DropdownMenuItem(child: Text(e.name), value: e.id);
      },
    ).toList();

    dropDownList.insert(
        0, DropdownMenuItem(child: Text("+ New Flow"), value: "new"));

    return Scaffold(
      appBar: AppBar(
        backgroundColor: Colors.black87,
        actions: [
          TextButton(
              onPressed: () {
                store.store.msgDeploy("deploy", timeout: Duration(minutes: 1));
              },
              child: Text("Deploy")),
          TextButton(
              onPressed: () {
                store.store.msgUnDeploy("undeploy");
              },
              child: Text("UnDeploy")),
          TextButton(
            child: Text("debug"),
            onPressed: () {
              store.store.msgDebug("debug");
            },
          ),
          TextButton(
            child: Text("reset zoom"),
            onPressed: () {
              _transformationController.value = Matrix4.identity();
            },
          ),
          DropdownButton(
              items: dropDownList,
              onChanged: (value) {
                store.store.msgLoadGraph(value.toString());
              })
        ],
      ),
      body: Stack(alignment: Alignment.bottomRight, children: [
        InteractiveViewer(
          panEnabled: true,
          clipBehavior: Clip.none,
          minScale: 0.1,
          maxScale: 10,
          // boundaryMargin: EdgeInsets.all(double.infinity),
          constrained: false,
          transformationController: _transformationController,
          // onInteractionStart: _onInteractionStart,
          child: LayoutBuilder(
            builder: (context, constraints) {
              return Container(
                height: MediaQuery.of(context).size.height + height,
                width: MediaQuery.of(context).size.width + width,
                decoration: BoxDecoration(color: Colors.blueGrey[900]),
                child: EventListener(),
              );
            },
          ),
        ),
        // Image.asset(
        //   "assets/logo-full-small.png",
        //   height: 100,
        // ),
      ]),
    );
  }
}
