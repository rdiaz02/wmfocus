#!/usr/bin/jq -rf

.monitors[] |
.focusedDesktopId as $d |
.desktops[] |
select(.id==$d) |
.focusedNodeId as $fw |
getpath(paths(.client?!=null and .hidden==false)) |
.id as $w |
.client |
(if .state == "floating" then .floatingRectangle else .tiledRectangle end) |
"\($w) \(.x) \(.y) \(.width) \(.height) \($fw==$w)"
