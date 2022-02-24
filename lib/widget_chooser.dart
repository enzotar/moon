import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/widgets/text_input.dart';
import 'package:moon/widget_input.dart';
import 'package:moon/widget_output.dart';
import 'package:moon/commands/add_pubkey.dart';
import 'package:moon/commands/airdrop.dart';
import 'package:moon/commands/const.dart';
import 'package:moon/commands/create_account.dart';
import 'package:moon/commands/create_token.dart';
import 'package:moon/commands/generate_keypair.dart';
import 'package:moon/nodes/command_widget.dart';
import 'package:moon/commands/print.dart';
import 'package:moon/commands/transfer.dart';

import './widgets/block.dart';

import 'dummy_edge_handle.dart';
import "logger.dart";

HookConsumerWidget WidgetChooser([
  nodeType,
  data,
  children,
  inputNodes,
  outputNodes,
  selectedNode,
  dimensions,
  storedContext,
]) {
  HookConsumerWidget? _widget;

  switch (nodeType) {
    case "WidgetBlock":
      {
        _widget = Block(
          node: data,
          children: children,
          // key: ObjectKey(data.nodeKey),
          selected: false,
        );
      }
      break;

    case "WidgetTextInput":
      {
        log.v("adding text input");

        _widget = TextInput(
          node: data,
          children: children ?? [Text("nochild")],
          // key: ObjectKey(data.nodeKey),
          selected: false,
          // selectedNode: selectedNode,
        );
      }
      break;
    case "DummyEdgeHandle":
      {
        // log.v("adding text input");

        // _widget = DummyEdgeHandle(
        //   node: data,
        //   children: children ?? [Text("nochild")],
        //   // key: ObjectKey(data.nodeKey),
        //   selected: false,
        // );
      }
      break;
    case "WidgetInput":
      {
        // log.v("adding text input");

        // _widget = WidgetInput(
        //   children: children ?? [Text("nochild")],
        //   node: data,
        //   selected: false,
        // );
      }
      break;
    case "WidgetOutput":
      {
        // log.v("adding text input");

        // _widget = WidgetOutput(
        //   children: children ?? [Text("nochild")],
        //   node: data,
        //   selected: false,
        // );
      }
      break;
    //
    case "Const":
      {
        _widget = Const(
          node: data,
          key: UniqueKey(),
          selected: false,
          inputs: inputNodes,
          outputs: outputNodes,
        );
      }
      break;
    case "Print":
    case "JsonExtract":
    case "HttpRequest":
    case "IpfsUpload":
    //
    case "CreateToken":
    case "AddPubkey":
    case "CreateAccount":
    case "GenerateKeypair":
    case "MintToken":
    case "Transfer":
    case "RequestAirdrop":
    case "GetBalance":
    //
    case "CreateMetadataAccounts":
    case "CreateMasterEdition":
    case "UpdateMetadataAccounts":
    case "Utilize":
    case "ApproveUseAuthority":
    case "GetLeftUses":
    case "ArweaveUpload":
      {
        _widget = CommandWidget(
          node: data,
          key: UniqueKey(),
          selected: false,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
        );
      }
      break;
    default:
      {
        // return Text("No Matching cases: ${data.properties}");
      }
  }

  return _widget as HookConsumerWidget;
}
// }
