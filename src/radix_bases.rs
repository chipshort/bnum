/*
Code adapted from the Rust `num_bigint` library: https://docs.rs/num-bigint/latest/num_bigint/ used under the MIT license.
The original license file and copyright notice for this project can be found in this project's root at licenses/LICENSE-num-bigint.
*/

use crate::Digit;

#[cfg(feature = "u8_digit")]
mod bases {
    pub static BASES_HALF: &[(u8, usize)] = &[
        (0, 0), // 0
        (0, 0), // 1
        (0, 0), // 2
        (9, 2), // 3
        (0, 0), // 4
        (5, 1), // 5
        (6, 1), // 6
        (7, 1), // 7
        (0, 0), // 8
        (9, 1), // 9
        (10, 1), // 10
        (11, 1), // 11
        (12, 1), // 12
        (13, 1), // 13
        (14, 1), // 14
        (15, 1), // 15
        (0, 0), // 16
        (17, 1), // 17
        (18, 1), // 18
        (19, 1), // 19
        (20, 1), // 20
        (21, 1), // 21
        (22, 1), // 22
        (23, 1), // 23
        (24, 1), // 24
        (25, 1), // 25
        (26, 1), // 26
        (27, 1), // 27
        (28, 1), // 28
        (29, 1), // 29
        (30, 1), // 30
        (31, 1), // 31
        (0, 0), // 32
        (33, 1), // 33
        (34, 1), // 34
        (35, 1), // 35
        (36, 1), // 36
        (37, 1), // 37
        (38, 1), // 38
        (39, 1), // 39
        (40, 1), // 40
        (41, 1), // 41
        (42, 1), // 42
        (43, 1), // 43
        (44, 1), // 44
        (45, 1), // 45
        (46, 1), // 46
        (47, 1), // 47
        (48, 1), // 48
        (49, 1), // 49
        (50, 1), // 50
        (51, 1), // 51
        (52, 1), // 52
        (53, 1), // 53
        (54, 1), // 54
        (55, 1), // 55
        (56, 1), // 56
        (57, 1), // 57
        (58, 1), // 58
        (59, 1), // 59
        (60, 1), // 60
        (61, 1), // 61
        (62, 1), // 62
        (63, 1), // 63
        (0, 0), // 64
        (65, 1), // 65
        (66, 1), // 66
        (67, 1), // 67
        (68, 1), // 68
        (69, 1), // 69
        (70, 1), // 70
        (71, 1), // 71
        (72, 1), // 72
        (73, 1), // 73
        (74, 1), // 74
        (75, 1), // 75
        (76, 1), // 76
        (77, 1), // 77
        (78, 1), // 78
        (79, 1), // 79
        (80, 1), // 80
        (81, 1), // 81
        (82, 1), // 82
        (83, 1), // 83
        (84, 1), // 84
        (85, 1), // 85
        (86, 1), // 86
        (87, 1), // 87
        (88, 1), // 88
        (89, 1), // 89
        (90, 1), // 90
        (91, 1), // 91
        (92, 1), // 92
        (93, 1), // 93
        (94, 1), // 94
        (95, 1), // 95
        (96, 1), // 96
        (97, 1), // 97
        (98, 1), // 98
        (99, 1), // 99
        (100, 1), // 100
        (101, 1), // 101
        (102, 1), // 102
        (103, 1), // 103
        (104, 1), // 104
        (105, 1), // 105
        (106, 1), // 106
        (107, 1), // 107
        (108, 1), // 108
        (109, 1), // 109
        (110, 1), // 110
        (111, 1), // 111
        (112, 1), // 112
        (113, 1), // 113
        (114, 1), // 114
        (115, 1), // 115
        (116, 1), // 116
        (117, 1), // 117
        (118, 1), // 118
        (119, 1), // 119
        (120, 1), // 120
        (121, 1), // 121
        (122, 1), // 122
        (123, 1), // 123
        (124, 1), // 124
        (125, 1), // 125
        (126, 1), // 126
        (127, 1), // 127
        (0, 0), // 128
        (129, 1), // 129
        (130, 1), // 130
        (131, 1), // 131
        (132, 1), // 132
        (133, 1), // 133
        (134, 1), // 134
        (135, 1), // 135
        (136, 1), // 136
        (137, 1), // 137
        (138, 1), // 138
        (139, 1), // 139
        (140, 1), // 140
        (141, 1), // 141
        (142, 1), // 142
        (143, 1), // 143
        (144, 1), // 144
        (145, 1), // 145
        (146, 1), // 146
        (147, 1), // 147
        (148, 1), // 148
        (149, 1), // 149
        (150, 1), // 150
        (151, 1), // 151
        (152, 1), // 152
        (153, 1), // 153
        (154, 1), // 154
        (155, 1), // 155
        (156, 1), // 156
        (157, 1), // 157
        (158, 1), // 158
        (159, 1), // 159
        (160, 1), // 160
        (161, 1), // 161
        (162, 1), // 162
        (163, 1), // 163
        (164, 1), // 164
        (165, 1), // 165
        (166, 1), // 166
        (167, 1), // 167
        (168, 1), // 168
        (169, 1), // 169
        (170, 1), // 170
        (171, 1), // 171
        (172, 1), // 172
        (173, 1), // 173
        (174, 1), // 174
        (175, 1), // 175
        (176, 1), // 176
        (177, 1), // 177
        (178, 1), // 178
        (179, 1), // 179
        (180, 1), // 180
        (181, 1), // 181
        (182, 1), // 182
        (183, 1), // 183
        (184, 1), // 184
        (185, 1), // 185
        (186, 1), // 186
        (187, 1), // 187
        (188, 1), // 188
        (189, 1), // 189
        (190, 1), // 190
        (191, 1), // 191
        (192, 1), // 192
        (193, 1), // 193
        (194, 1), // 194
        (195, 1), // 195
        (196, 1), // 196
        (197, 1), // 197
        (198, 1), // 198
        (199, 1), // 199
        (200, 1), // 200
        (201, 1), // 201
        (202, 1), // 202
        (203, 1), // 203
        (204, 1), // 204
        (205, 1), // 205
        (206, 1), // 206
        (207, 1), // 207
        (208, 1), // 208
        (209, 1), // 209
        (210, 1), // 210
        (211, 1), // 211
        (212, 1), // 212
        (213, 1), // 213
        (214, 1), // 214
        (215, 1), // 215
        (216, 1), // 216
        (217, 1), // 217
        (218, 1), // 218
        (219, 1), // 219
        (220, 1), // 220
        (221, 1), // 221
        (222, 1), // 222
        (223, 1), // 223
        (224, 1), // 224
        (225, 1), // 225
        (226, 1), // 226
        (227, 1), // 227
        (228, 1), // 228
        (229, 1), // 229
        (230, 1), // 230
        (231, 1), // 231
        (232, 1), // 232
        (233, 1), // 233
        (234, 1), // 234
        (235, 1), // 235
        (236, 1), // 236
        (237, 1), // 237
        (238, 1), // 238
        (239, 1), // 239
        (240, 1), // 240
        (241, 1), // 241
        (242, 1), // 242
        (243, 1), // 243
        (244, 1), // 244
        (245, 1), // 245
        (246, 1), // 246
        (247, 1), // 247
        (248, 1), // 248
        (249, 1), // 249
        (250, 1), // 250
        (251, 1), // 251
        (252, 1), // 252
        (253, 1), // 253
        (254, 1), // 254
        (255, 1), // 255
        (0, 0), // 256
    ];
    
    pub(crate) static BASES_DIGIT: &[(u8, usize)] = &[
        (0, 0), // 0
        (0, 0), // 1
        (0, 0), // 2
        (243, 5), // 3
        (0, 0), // 4
        (125, 3), // 5
        (216, 3), // 6
        (49, 2), // 7
        (0, 0), // 8
        (81, 2), // 9
        (100, 2), // 10
        (121, 2), // 11
        (144, 2), // 12
        (169, 2), // 13
        (196, 2), // 14
        (225, 2), // 15
        (0, 0), // 16
        (17, 1), // 17
        (18, 1), // 18
        (19, 1), // 19
        (20, 1), // 20
        (21, 1), // 21
        (22, 1), // 22
        (23, 1), // 23
        (24, 1), // 24
        (25, 1), // 25
        (26, 1), // 26
        (27, 1), // 27
        (28, 1), // 28
        (29, 1), // 29
        (30, 1), // 30
        (31, 1), // 31
        (0, 0), // 32
        (33, 1), // 33
        (34, 1), // 34
        (35, 1), // 35
        (36, 1), // 36
        (37, 1), // 37
        (38, 1), // 38
        (39, 1), // 39
        (40, 1), // 40
        (41, 1), // 41
        (42, 1), // 42
        (43, 1), // 43
        (44, 1), // 44
        (45, 1), // 45
        (46, 1), // 46
        (47, 1), // 47
        (48, 1), // 48
        (49, 1), // 49
        (50, 1), // 50
        (51, 1), // 51
        (52, 1), // 52
        (53, 1), // 53
        (54, 1), // 54
        (55, 1), // 55
        (56, 1), // 56
        (57, 1), // 57
        (58, 1), // 58
        (59, 1), // 59
        (60, 1), // 60
        (61, 1), // 61
        (62, 1), // 62
        (63, 1), // 63
        (0, 0), // 64
        (65, 1), // 65
        (66, 1), // 66
        (67, 1), // 67
        (68, 1), // 68
        (69, 1), // 69
        (70, 1), // 70
        (71, 1), // 71
        (72, 1), // 72
        (73, 1), // 73
        (74, 1), // 74
        (75, 1), // 75
        (76, 1), // 76
        (77, 1), // 77
        (78, 1), // 78
        (79, 1), // 79
        (80, 1), // 80
        (81, 1), // 81
        (82, 1), // 82
        (83, 1), // 83
        (84, 1), // 84
        (85, 1), // 85
        (86, 1), // 86
        (87, 1), // 87
        (88, 1), // 88
        (89, 1), // 89
        (90, 1), // 90
        (91, 1), // 91
        (92, 1), // 92
        (93, 1), // 93
        (94, 1), // 94
        (95, 1), // 95
        (96, 1), // 96
        (97, 1), // 97
        (98, 1), // 98
        (99, 1), // 99
        (100, 1), // 100
        (101, 1), // 101
        (102, 1), // 102
        (103, 1), // 103
        (104, 1), // 104
        (105, 1), // 105
        (106, 1), // 106
        (107, 1), // 107
        (108, 1), // 108
        (109, 1), // 109
        (110, 1), // 110
        (111, 1), // 111
        (112, 1), // 112
        (113, 1), // 113
        (114, 1), // 114
        (115, 1), // 115
        (116, 1), // 116
        (117, 1), // 117
        (118, 1), // 118
        (119, 1), // 119
        (120, 1), // 120
        (121, 1), // 121
        (122, 1), // 122
        (123, 1), // 123
        (124, 1), // 124
        (125, 1), // 125
        (126, 1), // 126
        (127, 1), // 127
        (0, 0), // 128
        (129, 1), // 129
        (130, 1), // 130
        (131, 1), // 131
        (132, 1), // 132
        (133, 1), // 133
        (134, 1), // 134
        (135, 1), // 135
        (136, 1), // 136
        (137, 1), // 137
        (138, 1), // 138
        (139, 1), // 139
        (140, 1), // 140
        (141, 1), // 141
        (142, 1), // 142
        (143, 1), // 143
        (144, 1), // 144
        (145, 1), // 145
        (146, 1), // 146
        (147, 1), // 147
        (148, 1), // 148
        (149, 1), // 149
        (150, 1), // 150
        (151, 1), // 151
        (152, 1), // 152
        (153, 1), // 153
        (154, 1), // 154
        (155, 1), // 155
        (156, 1), // 156
        (157, 1), // 157
        (158, 1), // 158
        (159, 1), // 159
        (160, 1), // 160
        (161, 1), // 161
        (162, 1), // 162
        (163, 1), // 163
        (164, 1), // 164
        (165, 1), // 165
        (166, 1), // 166
        (167, 1), // 167
        (168, 1), // 168
        (169, 1), // 169
        (170, 1), // 170
        (171, 1), // 171
        (172, 1), // 172
        (173, 1), // 173
        (174, 1), // 174
        (175, 1), // 175
        (176, 1), // 176
        (177, 1), // 177
        (178, 1), // 178
        (179, 1), // 179
        (180, 1), // 180
        (181, 1), // 181
        (182, 1), // 182
        (183, 1), // 183
        (184, 1), // 184
        (185, 1), // 185
        (186, 1), // 186
        (187, 1), // 187
        (188, 1), // 188
        (189, 1), // 189
        (190, 1), // 190
        (191, 1), // 191
        (192, 1), // 192
        (193, 1), // 193
        (194, 1), // 194
        (195, 1), // 195
        (196, 1), // 196
        (197, 1), // 197
        (198, 1), // 198
        (199, 1), // 199
        (200, 1), // 200
        (201, 1), // 201
        (202, 1), // 202
        (203, 1), // 203
        (204, 1), // 204
        (205, 1), // 205
        (206, 1), // 206
        (207, 1), // 207
        (208, 1), // 208
        (209, 1), // 209
        (210, 1), // 210
        (211, 1), // 211
        (212, 1), // 212
        (213, 1), // 213
        (214, 1), // 214
        (215, 1), // 215
        (216, 1), // 216
        (217, 1), // 217
        (218, 1), // 218
        (219, 1), // 219
        (220, 1), // 220
        (221, 1), // 221
        (222, 1), // 222
        (223, 1), // 223
        (224, 1), // 224
        (225, 1), // 225
        (226, 1), // 226
        (227, 1), // 227
        (228, 1), // 228
        (229, 1), // 229
        (230, 1), // 230
        (231, 1), // 231
        (232, 1), // 232
        (233, 1), // 233
        (234, 1), // 234
        (235, 1), // 235
        (236, 1), // 236
        (237, 1), // 237
        (238, 1), // 238
        (239, 1), // 239
        (240, 1), // 240
        (241, 1), // 241
        (242, 1), // 242
        (243, 1), // 243
        (244, 1), // 244
        (245, 1), // 245
        (246, 1), // 246
        (247, 1), // 247
        (248, 1), // 248
        (249, 1), // 249
        (250, 1), // 250
        (251, 1), // 251
        (252, 1), // 252
        (253, 1), // 253
        (254, 1), // 254
        (255, 1), // 255
        (0, 0), // 256
    ];

	pub const RADIX_BASE_10: (crate::Digit, usize) = (100, 2);
}

#[cfg(not(feature = "u8_digit"))]
mod bases {
    pub static BASES_HALF: &[(u64, usize)] = &[
        (0, 0), // 0
        (0, 0), // 1
        (0, 0), // 2
        (3486784401, 20), // 3
        (0, 0), // 4
        (1220703125, 13), // 5
        (2176782336, 12), // 6
        (1977326743, 11), // 7
        (0, 0), // 8
        (3486784401, 10), // 9
        (1000000000, 9), // 10
        (2357947691, 9), // 11
        (429981696, 8), // 12
        (815730721, 8), // 13
        (1475789056, 8), // 14
        (2562890625, 8), // 15
        (0, 0), // 16
        (410338673, 7), // 17
        (612220032, 7), // 18
        (893871739, 7), // 19
        (1280000000, 7), // 20
        (1801088541, 7), // 21
        (2494357888, 7), // 22
        (3404825447, 7), // 23
        (191102976, 6), // 24
        (244140625, 6), // 25
        (308915776, 6), // 26
        (387420489, 6), // 27
        (481890304, 6), // 28
        (594823321, 6), // 29
        (729000000, 6), // 30
        (887503681, 6), // 31
        (0, 0), // 32
        (1291467969, 6), // 33
        (1544804416, 6), // 34
        (1838265625, 6), // 35
        (2176782336, 6), // 36
        (2565726409, 6), // 37
        (3010936384, 6), // 38
        (3518743761, 6), // 39
        (4096000000, 6), // 40
        (115856201, 5), // 41
        (130691232, 5), // 42
        (147008443, 5), // 43
        (164916224, 5), // 44
        (184528125, 5), // 45
        (205962976, 5), // 46
        (229345007, 5), // 47
        (254803968, 5), // 48
        (282475249, 5), // 49
        (312500000, 5), // 50
        (345025251, 5), // 51
        (380204032, 5), // 52
        (418195493, 5), // 53
        (459165024, 5), // 54
        (503284375, 5), // 55
        (550731776, 5), // 56
        (601692057, 5), // 57
        (656356768, 5), // 58
        (714924299, 5), // 59
        (777600000, 5), // 60
        (844596301, 5), // 61
        (916132832, 5), // 62
        (992436543, 5), // 63
        (0, 0), // 64
        (1160290625, 5), // 65
        (1252332576, 5), // 66
        (1350125107, 5), // 67
        (1453933568, 5), // 68
        (1564031349, 5), // 69
        (1680700000, 5), // 70
        (1804229351, 5), // 71
        (1934917632, 5), // 72
        (2073071593, 5), // 73
        (2219006624, 5), // 74
        (2373046875, 5), // 75
        (2535525376, 5), // 76
        (2706784157, 5), // 77
        (2887174368, 5), // 78
        (3077056399, 5), // 79
        (3276800000, 5), // 80
        (3486784401, 5), // 81
        (3707398432, 5), // 82
        (3939040643, 5), // 83
        (4182119424, 5), // 84
        (52200625, 4), // 85
        (54700816, 4), // 86
        (57289761, 4), // 87
        (59969536, 4), // 88
        (62742241, 4), // 89
        (65610000, 4), // 90
        (68574961, 4), // 91
        (71639296, 4), // 92
        (74805201, 4), // 93
        (78074896, 4), // 94
        (81450625, 4), // 95
        (84934656, 4), // 96
        (88529281, 4), // 97
        (92236816, 4), // 98
        (96059601, 4), // 99
        (100000000, 4), // 100
        (104060401, 4), // 101
        (108243216, 4), // 102
        (112550881, 4), // 103
        (116985856, 4), // 104
        (121550625, 4), // 105
        (126247696, 4), // 106
        (131079601, 4), // 107
        (136048896, 4), // 108
        (141158161, 4), // 109
        (146410000, 4), // 110
        (151807041, 4), // 111
        (157351936, 4), // 112
        (163047361, 4), // 113
        (168896016, 4), // 114
        (174900625, 4), // 115
        (181063936, 4), // 116
        (187388721, 4), // 117
        (193877776, 4), // 118
        (200533921, 4), // 119
        (207360000, 4), // 120
        (214358881, 4), // 121
        (221533456, 4), // 122
        (228886641, 4), // 123
        (236421376, 4), // 124
        (244140625, 4), // 125
        (252047376, 4), // 126
        (260144641, 4), // 127
        (0, 0), // 128
        (276922881, 4), // 129
        (285610000, 4), // 130
        (294499921, 4), // 131
        (303595776, 4), // 132
        (312900721, 4), // 133
        (322417936, 4), // 134
        (332150625, 4), // 135
        (342102016, 4), // 136
        (352275361, 4), // 137
        (362673936, 4), // 138
        (373301041, 4), // 139
        (384160000, 4), // 140
        (395254161, 4), // 141
        (406586896, 4), // 142
        (418161601, 4), // 143
        (429981696, 4), // 144
        (442050625, 4), // 145
        (454371856, 4), // 146
        (466948881, 4), // 147
        (479785216, 4), // 148
        (492884401, 4), // 149
        (506250000, 4), // 150
        (519885601, 4), // 151
        (533794816, 4), // 152
        (547981281, 4), // 153
        (562448656, 4), // 154
        (577200625, 4), // 155
        (592240896, 4), // 156
        (607573201, 4), // 157
        (623201296, 4), // 158
        (639128961, 4), // 159
        (655360000, 4), // 160
        (671898241, 4), // 161
        (688747536, 4), // 162
        (705911761, 4), // 163
        (723394816, 4), // 164
        (741200625, 4), // 165
        (759333136, 4), // 166
        (777796321, 4), // 167
        (796594176, 4), // 168
        (815730721, 4), // 169
        (835210000, 4), // 170
        (855036081, 4), // 171
        (875213056, 4), // 172
        (895745041, 4), // 173
        (916636176, 4), // 174
        (937890625, 4), // 175
        (959512576, 4), // 176
        (981506241, 4), // 177
        (1003875856, 4), // 178
        (1026625681, 4), // 179
        (1049760000, 4), // 180
        (1073283121, 4), // 181
        (1097199376, 4), // 182
        (1121513121, 4), // 183
        (1146228736, 4), // 184
        (1171350625, 4), // 185
        (1196883216, 4), // 186
        (1222830961, 4), // 187
        (1249198336, 4), // 188
        (1275989841, 4), // 189
        (1303210000, 4), // 190
        (1330863361, 4), // 191
        (1358954496, 4), // 192
        (1387488001, 4), // 193
        (1416468496, 4), // 194
        (1445900625, 4), // 195
        (1475789056, 4), // 196
        (1506138481, 4), // 197
        (1536953616, 4), // 198
        (1568239201, 4), // 199
        (1600000000, 4), // 200
        (1632240801, 4), // 201
        (1664966416, 4), // 202
        (1698181681, 4), // 203
        (1731891456, 4), // 204
        (1766100625, 4), // 205
        (1800814096, 4), // 206
        (1836036801, 4), // 207
        (1871773696, 4), // 208
        (1908029761, 4), // 209
        (1944810000, 4), // 210
        (1982119441, 4), // 211
        (2019963136, 4), // 212
        (2058346161, 4), // 213
        (2097273616, 4), // 214
        (2136750625, 4), // 215
        (2176782336, 4), // 216
        (2217373921, 4), // 217
        (2258530576, 4), // 218
        (2300257521, 4), // 219
        (2342560000, 4), // 220
        (2385443281, 4), // 221
        (2428912656, 4), // 222
        (2472973441, 4), // 223
        (2517630976, 4), // 224
        (2562890625, 4), // 225
        (2608757776, 4), // 226
        (2655237841, 4), // 227
        (2702336256, 4), // 228
        (2750058481, 4), // 229
        (2798410000, 4), // 230
        (2847396321, 4), // 231
        (2897022976, 4), // 232
        (2947295521, 4), // 233
        (2998219536, 4), // 234
        (3049800625, 4), // 235
        (3102044416, 4), // 236
        (3154956561, 4), // 237
        (3208542736, 4), // 238
        (3262808641, 4), // 239
        (3317760000, 4), // 240
        (3373402561, 4), // 241
        (3429742096, 4), // 242
        (3486784401, 4), // 243
        (3544535296, 4), // 244
        (3603000625, 4), // 245
        (3662186256, 4), // 246
        (3722098081, 4), // 247
        (3782742016, 4), // 248
        (3844124001, 4), // 249
        (3906250000, 4), // 250
        (3969126001, 4), // 251
        (4032758016, 4), // 252
        (4097152081, 4), // 253
        (4162314256, 4), // 254
        (4228250625, 4), // 255
        (0, 0), // 256
    ];

    pub static BASES_DIGIT: &[(u64, usize)] = &[
        (0, 0), // 0
        (0, 0), // 1
        (0, 0), // 2
        (12157665459056928801, 40), // 3
        (0, 0), // 4
        (7450580596923828125, 27), // 5
        (4738381338321616896, 24), // 6
        (3909821048582988049, 22), // 7
        (0, 0), // 8
        (12157665459056928801, 20), // 9
        (10000000000000000000, 19), // 10
        (5559917313492231481, 18), // 11
        (2218611106740436992, 17), // 12
        (8650415919381337933, 17), // 13
        (2177953337809371136, 16), // 14
        (6568408355712890625, 16), // 15
        (0, 0), // 16
        (2862423051509815793, 15), // 17
        (6746640616477458432, 15), // 18
        (15181127029874798299, 15), // 19
        (1638400000000000000, 14), // 20
        (3243919932521508681, 14), // 21
        (6221821273427820544, 14), // 22
        (11592836324538749809, 14), // 23
        (876488338465357824, 13), // 24
        (1490116119384765625, 13), // 25
        (2481152873203736576, 13), // 26
        (4052555153018976267, 13), // 27
        (6502111422497947648, 13), // 28
        (10260628712958602189, 13), // 29
        (15943230000000000000, 13), // 30
        (787662783788549761, 12), // 31
        (0, 0), // 32
        (1667889514952984961, 12), // 33
        (2386420683693101056, 12), // 34
        (3379220508056640625, 12), // 35
        (4738381338321616896, 12), // 36
        (6582952005840035281, 12), // 37
        (9065737908494995456, 12), // 38
        (12381557655576425121, 12), // 39
        (16777216000000000000, 12), // 40
        (550329031716248441, 11), // 41
        (717368321110468608, 11), // 42
        (929293739471222707, 11), // 43
        (1196683881290399744, 11), // 44
        (1532278301220703125, 11), // 45
        (1951354384207722496, 11), // 46
        (2472159215084012303, 11), // 47
        (3116402981210161152, 11), // 48
        (3909821048582988049, 11), // 49
        (4882812500000000000, 11), // 50
        (6071163615208263051, 11), // 51
        (7516865509350965248, 11), // 52
        (9269035929372191597, 11), // 53
        (11384956040305711104, 11), // 54
        (13931233916552734375, 11), // 55
        (16985107389382393856, 11), // 56
        (362033331456891249, 10), // 57
        (430804206899405824, 10), // 58
        (511116753300641401, 10), // 59
        (604661760000000000, 10), // 60
        (713342911662882601, 10), // 61
        (839299365868340224, 10), // 62
        (984930291881790849, 10), // 63
        (0, 0), // 64
        (1346274334462890625, 10), // 65
        (1568336880910795776, 10), // 66
        (1822837804551761449, 10), // 67
        (2113922820157210624, 10), // 68
        (2446194060654759801, 10), // 69
        (2824752490000000000, 10), // 70
        (3255243551009881201, 10), // 71
        (3743906242624487424, 10), // 72
        (4297625829703557649, 10), // 73
        (4923990397355877376, 10), // 74
        (5631351470947265625, 10), // 75
        (6428888932339941376, 10), // 76
        (7326680472586200649, 10), // 77
        (8335775831236199424, 10), // 78
        (9468276082626847201, 10), // 79
        (10737418240000000000, 10), // 80
        (12157665459056928801, 10), // 81
        (13744803133596058624, 10), // 82
        (15516041187205853449, 10), // 83
        (17490122876598091776, 10), // 84
        (231616946283203125, 9), // 85
        (257327417311663616, 9), // 86
        (285544154243029527, 9), // 87
        (316478381828866048, 9), // 88
        (350356403707485209, 9), // 89
        (387420489000000000, 9), // 90
        (427929800129788411, 9), // 91
        (472161363286556672, 9), // 92
        (520411082988487293, 9), // 93
        (572994802228616704, 9), // 94
        (630249409724609375, 9), // 95
        (692533995824480256, 9), // 96
        (760231058654565217, 9), // 97
        (833747762130149888, 9), // 98
        (913517247483640899, 9), // 99
        (1000000000000000000, 9), // 100
        (1093685272684360901, 9), // 101
        (1195092568622310912, 9), // 102
        (1304773183829244583, 9), // 103
        (1423311812421484544, 9), // 104
        (1551328215978515625, 9), // 105
        (1689478959002692096, 9), // 106
        (1838459212420154507, 9), // 107
        (1999004627104432128, 9), // 108
        (2171893279442309389, 9), // 109
        (2357947691000000000, 9), // 110
        (2558036924386500591, 9), // 111
        (2773078757450186752, 9), // 112
        (3004041937984268273, 9), // 113
        (3251948521156637184, 9), // 114
        (3517876291919921875, 9), // 115
        (3802961274698203136, 9), // 116
        (4108400332687853397, 9), // 117
        (4435453859151328768, 9), // 118
        (4785448563124474679, 9), // 119
        (5159780352000000000, 9), // 120
        (5559917313492231481, 9), // 121
        (5987402799531080192, 9), // 122
        (6443858614676334363, 9), // 123
        (6930988311686938624, 9), // 124
        (7450580596923828125, 9), // 125
        (8004512848309157376, 9), // 126
        (8594754748609397887, 9), // 127
        (0, 0), // 128
        (9892530380752880769, 9), // 129
        (10604499373000000000, 9), // 130
        (11361656654439817571, 9), // 131
        (12166492167065567232, 9), // 132
        (13021612539908538853, 9), // 133
        (13929745610903012864, 9), // 134
        (14893745087865234375, 9), // 135
        (15916595351771938816, 9), // 136
        (17001416405572203977, 9), // 137
        (18151468971815029248, 9), // 138
        (139353667211683681, 8), // 139
        (147578905600000000, 8), // 140
        (156225851787813921, 8), // 141
        (165312903998914816, 8), // 142
        (174859124550883201, 8), // 143
        (184884258895036416, 8), // 144
        (195408755062890625, 8), // 145
        (206453783524884736, 8), // 146
        (218041257467152161, 8), // 147
        (230193853492166656, 8), // 148
        (242935032749128801, 8), // 149
        (256289062500000000, 8), // 150
        (270281038127131201, 8), // 151
        (284936905588473856, 8), // 152
        (300283484326400961, 8), // 153
        (316348490636206336, 8), // 154
        (333160561500390625, 8), // 155
        (350749278894882816, 8), // 156
        (369145194573386401, 8), // 157
        (388379855336079616, 8), // 158
        (408485828788939521, 8), // 159
        (429496729600000000, 8), // 160
        (451447246258894081, 8), // 161
        (474373168346071296, 8), // 162
        (498311414318121121, 8), // 163
        (523300059815673856, 8), // 164
        (549378366500390625, 8), // 165
        (576586811427594496, 8), // 166
        (604967116961135041, 8), // 167
        (634562281237118976, 8), // 168
        (665416609183179841, 8), // 169
        (697575744100000000, 8), // 170
        (731086699811838561, 8), // 171
        (765997893392859136, 8), // 172
        (802359178476091681, 8), // 173
        (840221879151902976, 8), // 174
        (879638824462890625, 8), // 175
        (920664383502155776, 8), // 176
        (963354501121950081, 8), // 177
        (1007766734259732736, 8), // 178
        (1053960288888713761, 8), // 179
        (1101996057600000000, 8), // 180
        (1151936657823500641, 8), // 181
        (1203846470694789376, 8), // 182
        (1257791680575160641, 8), // 183
        (1313840315232157696, 8), // 184
        (1372062286687890625, 8), // 185
        (1432529432742502656, 8), // 186
        (1495315559180183521, 8), // 187
        (1560496482665168896, 8), // 188
        (1628150074335205281, 8), // 189
        (1698356304100000000, 8), // 190
        (1771197285652216321, 8), // 191
        (1846757322198614016, 8), // 192
        (1925122952918976001, 8), // 193
        (2006383000160502016, 8), // 194
        (2090628617375390625, 8), // 195
        (2177953337809371136, 8), // 196
        (2268453123948987361, 8), // 197
        (2362226417735475456, 8), // 198
        (2459374191553118401, 8), // 199
        (2560000000000000000, 8), // 200
        (2664210032449121601, 8), // 201
        (2772113166407885056, 8), // 202
        (2883821021683985761, 8), // 203
        (2999448015365799936, 8), // 204
        (3119111417625390625, 8), // 205
        (3242931408352297216, 8), // 206
        (3371031134626313601, 8), // 207
        (3503536769037500416, 8), // 208
        (3640577568861717121, 8), // 209
        (3782285936100000000, 8), // 210
        (3928797478390152481, 8), // 211
        (4080251070798954496, 8), // 212
        (4236788918503437921, 8), // 213
        (4398556620369715456, 8), // 214
        (4565703233437890625, 8), // 215
        (4738381338321616896, 8), // 216
        (4916747105530914241, 8), // 217
        (5100960362726891776, 8), // 218
        (5291184662917065441, 8), // 219
        (5487587353600000000, 8), // 220
        (5690339646868044961, 8), // 221
        (5899616690476974336, 8), // 222
        (6115597639891380481, 8), // 223
        (6338465731314712576, 8), // 224
        (6568408355712890625, 8), // 225
        (6805617133840466176, 8), // 226
        (7050287992278341281, 8), // 227
        (7302621240492097536, 8), // 228
        (7562821648920027361, 8), // 229
        (7831098528100000000, 8), // 230
        (8107665808844335041, 8), // 231
        (8392742123471896576, 8), // 232
        (8686550888106661441, 8), // 233
        (8989320386052055296, 8), // 234
        (9301283852250390625, 8), // 235
        (9622679558836781056, 8), // 236
        (9953750901796946721, 8), // 237
        (10294746488738365696, 8), // 238
        (10645920227784266881, 8), // 239
        (11007531417600000000, 8), // 240
        (11379844838561358721, 8), // 241
        (11763130845074473216, 8), // 242
        (12157665459056928801, 8), // 243
        (12563730464589807616, 8), // 244
        (12981613503750390625, 8), // 245
        (13411608173635297536, 8), // 246
        (13854014124583882561, 8), // 247
        (14309137159611744256, 8), // 248
        (14777289335064248001, 8), // 249
        (15258789062500000000, 8), // 250
        (15753961211814252001, 8), // 251
        (16263137215612256256, 8), // 252
        (16786655174842630561, 8), // 253
        (17324859965700833536, 8), // 254
        (17878103347812890625, 8), // 255
        (0, 0), // 256
    ];

	pub const RADIX_BASE_10: (crate::Digit, usize) = (10000000000000000000, 19);
}

pub use bases::RADIX_BASE_10;

#[inline]
pub fn get_radix_base<const HALF: bool>(radix: u32) -> (Digit, usize) {
    if HALF {
        bases::BASES_HALF[radix as usize]
    } else {
        bases::BASES_DIGIT[radix as usize]
    }
}