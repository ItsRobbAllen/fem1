# Embedded M-VAVE stock firmware

F'Em 1 includes the unmodified M-VAVE FM-1 V15 recovery firmware,
`FM-1_015.fwsc` (699,956 bytes).

Copyright (c) 2026 珠海笙科智能科技有限公司 (M-VAVE).

Source repository: https://github.com/m-vave/fm1_open
Pinned source commit: `4c7496daac04a95ff10d1afdcc8f9ead2dbabc41`
Source file: `sdk/firmware/FM-1_015.fwsc`
SHA-256: `db1642b2b6fa5c2cccb11ffd13878068bb28601678d3644049f99dc40e7edb8a`

M-VAVE separately licenses this recovery image under Apache License 2.0.
See the complete LICENSE-2.0.txt and original THIRD_PARTY.md, especially
“Boot/update resources and recovery firmware”. The firmware is embedded as-is;
the emulator supplies its own initial patch bank when virtual user flash is blank.

This permission is specific to the V15 recovery image. It is not a blanket
permission for other M-VAVE firmware versions or the separate proprietary SDK.
F'Em 1 is an independent project; M-VAVE has not endorsed it.
