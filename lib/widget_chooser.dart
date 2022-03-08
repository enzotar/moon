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

SuperBlock WidgetChooser([nodeType, treeNode, inputNodes, outputNodes, parentId
    // storedContext,
    ]) {
  HookConsumerWidget? _widget;

  switch (nodeType) {
    case "WidgetBlock":
      {
        _widget = Block(
          // key: UniqueKey(),
          treeNode: treeNode,
        );
      }
      break;

    case "WidgetTextInput":
      {
        log.v("adding text input");

        _widget = TextInput(
          // key: UniqueKey(),
          treeNode: treeNode,
          parentId: parentId,
          // context: storedContext,
        );
      }
      break;
    case "DummyEdgeHandle":
      {
        print("trying to add DummyEdgeHandle");

        // _widget = DummyEdgeHandle(
        //   key: UniqueKey(),
        //   treeNode: treeNode,
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
        _widget = CommandWidget(
          // key: UniqueKey(),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          child: Const(treeNode: treeNode),
          parentId: parentId,
        );
      }
      // {
      //   _widget = Const(
      //     key: UniqueKey(),
      //     treeNode: treeNode,
      //     inputs: inputNodes,
      //     outputs: outputNodes,
      //   );
      // }
      break;
    case "Print":
      {
        _widget = CommandWidget(
          // key: UniqueKey(),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          child: Print(treeNode: treeNode),
          parentId: parentId,
        );
      }
      break;
    case "JsonExtract":
    case "HttpRequest":
    case "IpfsUpload":
    case "IpfsNftUpload":
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
          // key: UniqueKey(),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          parentId: parentId,
        );
      }
      break;
    default:
      {
        // return Text("No Matching cases: ${data.properties}");
      }
  }

  return _widget as SuperBlock;
}
// }
