import Cocoa
import FlutterMacOS

public class Plugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result(nil)
  }
}
// <rid:prevent_tree_shake Start>
func dummyCallsToPreventTreeShaking() {
    _export_dart_enum_SolanaNet();
    _to_dart_for_GraphEntry();
    rid_graphentry_debug(nil);
    rid_graphentry_debug_pretty(nil);
    rid_graphentry_id(nil);
    rid_graphentry_id_len(nil);
    rid_graphentry_name(nil);
    rid_graphentry_name_len(nil);
    rid_cstring_free(nil);
    rid_init_msg_isolate(0);
    rid_init_reply_isolate(0);
    rid_solananet_debug(0);
    rid_solananet_debug_pretty(0);
    _export_dart_enum_NodeChangeKind();
    _export_dart_enum_RunStateView();
    _export_dart_enum_NodeViewType();
    _export_dart_enum_ViewEdgeType();
    _to_dart_for_View();
    rid_view_debug(nil);
    rid_view_debug_pretty(nil);
    // __include_dart_for_vec_string();
    rid_view_graph_entry(nil);
    rid_view_nodes(nil);
    rid_view_flow_edges(nil);
    rid_view_selected_node_ids(nil);
    rid_view_selection(nil);
    rid_view_command(nil);
    rid_view_text_commands(nil);
    rid_view_graph_list(nil);
    rid_view_highlighted(nil);
    rid_view_transform(nil);
    rid_view_transform_screenshot(nil);
    rid_view_bookmarks(nil);
    rid_view_solana_net(nil);
    rid_view_ui_state_debug(nil);
    rid_len_vec_string(nil);
    rid_get_item_vec_string(nil, 0);
    rid_export_rid_len_hash_map_string_bookmarkview(nil);
    rid_export_rid_get_hash_map_string_bookmarkview(nil, nil);
    rid_export_rid_contains_key_hash_map_string_bookmarkview(nil, nil);
    rid_export_rid_keys_hash_map_string_bookmarkview(nil);
    __include_dart_for_ridvec_string();
    rid_free_ridvec_string(RidVec_Pointer_String());
    rid_get_item_ridvec_string(RidVec_Pointer_String(), 0);
    rid_export_rid_len_hash_map_string_edgeview(nil);
    rid_export_rid_get_hash_map_string_edgeview(nil, nil);
    rid_export_rid_contains_key_hash_map_string_edgeview(nil, nil);
    rid_export_rid_keys_hash_map_string_edgeview(nil);
    rid_len_vec_widgettextcommand(nil);
    rid_get_item_vec_widgettextcommand(nil, 0);
    rid_len_vec_graphentry(nil);
    rid_get_item_vec_graphentry(nil, 0);
    rid_export_rid_len_hash_map_string_nodeview(nil);
    rid_export_rid_get_hash_map_string_nodeview(nil, nil);
    rid_export_rid_contains_key_hash_map_string_nodeview(nil, nil);
    rid_export_rid_keys_hash_map_string_nodeview(nil);
    _to_dart_for_DebugData();
    rid_debugdata_ui_state(nil);
    rid_debugdata_ui_state_len(nil);
    rid_debugdata_mapping_kind(nil);
    rid_debugdata_mapping_kind_len(nil);
    _to_dart_for_Ratio();
    rid_ratio_numer(nil);
    rid_ratio_denom(nil);
    _to_dart_for_LastViewChanges();
    __include_dart_for_hash_map_string_nodechange();
    rid_lastviewchanges_changed_nodes_ids(nil);
    rid_lastviewchanges_changed_flow_edges_ids(nil);
    rid_lastviewchanges_is_selected_node_ids_changed(nil);
    rid_lastviewchanges_is_selection_changed(nil);
    rid_lastviewchanges_is_command_changed(nil);
    rid_lastviewchanges_is_text_commands_changed(nil);
    rid_lastviewchanges_is_graph_list_changed(nil);
    rid_lastviewchanges_is_highlighted_changed(nil);
    rid_lastviewchanges_is_transform_changed(nil);
    rid_lastviewchanges_is_transform_screenshot_changed(nil);
    rid_lastviewchanges_is_graph_changed(nil);
    rid_lastviewchanges_is_bookmark_changed(nil);
    rid_export_rid_len_hash_map_string_nodechange(nil);
    rid_export_rid_get_hash_map_string_nodechange(nil, nil);
    rid_export_rid_contains_key_hash_map_string_nodechange(nil, nil);
    rid_export_rid_keys_hash_map_string_nodechange(nil);
    _to_dart_for_Camera();
    rid_camera_x(nil);
    rid_camera_y(nil);
    rid_camera_scale(nil);
    _to_dart_for_NodeChange();
    rid_nodechange_debug(nil);
    rid_nodechange_debug_pretty(nil);
    rid_nodechange_kind(nil);
    rid_nodechangekind_debug(0);
    rid_nodechangekind_debug_pretty(0);
    _to_dart_for_Selection();
    rid_selection_is_active(nil);
    rid_selection_x1(nil);
    rid_selection_y1(nil);
    rid_selection_x2(nil);
    rid_selection_y2(nil);
    _to_dart_for_Command();
    rid_command_is_active(nil);
    rid_command_command(nil);
    rid_command_command_len(nil);
    _to_dart_for_WidgetTextCommand();
    // __include_dart_for_vec_textcommandoutput();
    rid_widgettextcommand_command_name(nil);
    rid_widgettextcommand_command_name_len(nil);
    rid_widgettextcommand_widget_name(nil);
    rid_widgettextcommand_widget_name_len(nil);
    rid_widgettextcommand_inputs(nil);
    rid_widgettextcommand_outputs(nil);
    rid_len_vec_textcommandoutput(nil);
    rid_get_item_vec_textcommandoutput(nil, 0);
    rid_len_vec_textcommandinput(nil);
    rid_get_item_vec_textcommandinput(nil, 0);
    _to_dart_for_TextCommandInput();
    rid_textcommandinput_name(nil);
    rid_textcommandinput_name_len(nil);
    rid_textcommandinput_acceptable_kinds(nil);
    _to_dart_for_TextCommandOutput();
    rid_textcommandoutput_name(nil);
    rid_textcommandoutput_name_len(nil);
    rid_textcommandoutput_kind(nil);
    rid_textcommandoutput_kind_len(nil);
    _to_dart_for_NodeView();
    rid_nodeview_index(nil);
    rid_nodeview_parent_id(nil);
    rid_nodeview_parent_id_len(nil);
    rid_nodeview_origin_x(nil);
    rid_nodeview_origin_y(nil);
    rid_nodeview_x(nil);
    rid_nodeview_y(nil);
    rid_nodeview_height(nil);
    rid_nodeview_width(nil);
    rid_nodeview_text(nil);
    rid_nodeview_text_len(nil);
    rid_nodeview_outbound_edges(nil);
    rid_nodeview_widget_type(nil);
    rid_nodeview_flow_inbound_edges(nil);
    rid_nodeview_flow_outbound_edges(nil);
    rid_nodeview_run_state(nil);
    rid_nodeview_elapsed_time(nil);
    rid_nodeview_error(nil);
    rid_nodeview_error_len(nil);
    rid_nodeview_print_output(nil);
    rid_nodeview_print_output_len(nil);
    rid_nodeview_additional_data(nil);
    rid_nodeview_additional_data_len(nil);
    rid_runstateview_debug(0);
    rid_runstateview_debug_pretty(0);
    rid_nodeviewtype_debug(0);
    rid_nodeviewtype_debug_pretty(0);
    _to_dart_for_EdgeView();
    rid_edgeview_from(nil);
    rid_edgeview_from_len(nil);
    rid_edgeview_to(nil);
    rid_edgeview_to_len(nil);
    rid_edgeview_edge_type(nil);
    rid_edgeview_from_coords_x(nil);
    rid_edgeview_from_coords_y(nil);
    rid_edgeview_to_coords_x(nil);
    rid_edgeview_to_coords_y(nil);
    _to_dart_for_BookmarkView();
    rid_bookmarkview_name(nil);
    rid_bookmarkview_name_len(nil);
    rid_bookmarkview_nodes(nil);
    _to_dart_for_Store();
    create_store();
    rid_store_unlock();
    rid_store_free();
    rid_store_view(nil);
    rid_store_last_view_changes(nil);
    _include_Store_field_wrappers();
    rid_msg_Initialize(0, nil);
    rid_msg_ResizeCanvas(0, nil);
    rid_msg_MouseEvent(0, nil);
    rid_msg_KeyboardEvent(0, nil);
    rid_msg_LoadGraph(0, nil);
    rid_msg_Debug(0, nil);
    rid_msg_SendJson(0, nil);
    rid_msg_StartInput(0, nil);
    rid_msg_StopInput(0, nil);
    rid_msg_SetText(0, nil);
    rid_msg_ApplyCommand(0, nil);
    rid_msg_ApplyAutocomplete(0, nil);
    rid_msg_Deploy(0, nil);
    rid_msg_UnDeploy(0, nil);
    rid_msg_Request(0, nil);
    rid_msg_Refresh(0, nil);
    rid_msg_Import(0, nil);
    rid_msg_Export(0, nil, nil);
    rid_msg_ResetZoom(0, nil);
    rid_msg_FitNodesToScreen(0, nil);
    rid_msg_CreateBookmark(0, nil);
    rid_msg_GotoBookmark(0, nil);
    rid_msg_DeleteBookmark(0, nil);
    rid_msg_ChangeSolanaNet(0, nil);
    rid_msg_BookmarkScreenshot(0, nil);
    rid_msg_UpdateDimensions(0, nil, nil, 0, 0);
    rid_msg_GenerateSeedPhrase(0, nil);
    rid_msg_RemoveNode(0, nil);
    rid_msg_SetMappingKind(0, nil);
}
// <rid:prevent_tree_shake End>