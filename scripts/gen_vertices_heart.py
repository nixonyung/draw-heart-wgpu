from random import choice

VERTICES = (
    (296, 110),
    (175, 130),
    (267, 226),
    (104, 192),
    (74, 272),
    (265, 319),
    (78, 382),
    (126, 517),
    (263, 489),
    (176, 592),
    (295, 583),
    (221, 653),
    (307, 639),
    (286, 721),
    (476, 649),
    (363, 788),
    (446, 855),
    (500, 892),
    (696, 738),
    (684, 647),
    (813, 613),
    (595, 537),
    (454, 575),
    (454, 476),
    (455, 348),
    (499, 209),
    (408, 147),
    (592, 307),
    (591, 149),
    (684, 115),
    (734, 239),
    (781, 115),
    (883, 176),
    (928, 278),
    (925, 373),
    (735, 425),
    (877, 508),
    (728, 500),
    (597, 449),
)

INDICES = (
    (0, 1, 2),
    (1, 3, 2),
    (3, 4, 2),
    (4, 5, 2),
    (4, 6, 5),
    (6, 7, 5),
    (7, 8, 5),
    (7, 9, 8),
    (9, 10, 8),
    (9, 11, 10),
    (11, 12, 10),
    (11, 13, 12),
    (12, 13, 14),
    (13, 15, 14),
    (15, 16, 14),
    (16, 17, 14),
    (17, 18, 14),
    (18, 19, 14),
    (18, 20, 19),
    (19, 20, 21),
    (19, 21, 22),
    (14, 19, 22),
    (12, 14, 22),
    (10, 12, 22),
    (10, 22, 23),
    (10, 22, 23),
    (10, 23, 8),
    (8, 23, 24),
    (8, 24, 5),
    (5, 24, 25),
    (5, 25, 2),
    (2, 25, 26),
    (2, 26, 0),
    (25, 24, 27),
    (25, 24, 27),
    (25, 27, 28),
    (28, 27, 29),
    (29, 27, 30),
    (29, 30, 31),
    (31, 30, 32),
    (32, 30, 33),
    (33, 30, 34),
    (30, 35, 34),
    (35, 36, 34),
    (35, 37, 36),
    (36, 37, 20),
    (37, 21, 20),
    (35, 21, 37),
    (38, 21, 35),
    (30, 38, 35),
    (30, 27, 38),
    (27, 24, 38),
    (24, 23, 38),
    (38, 23, 21),
    (23, 22, 21),
)

COLORS = [
    f"{15/255:.3}, {4/255:.3}, {6/255:.3}",
    f"{55/255:.3}, {28/255:.3}, {33/255:.3}",
    f"{95/255:.3}, {66/255:.3}, {75/255:.3}",
    f"{135/255:.3}, {111/255:.3}, {122/255:.3}",
    f"{175/255:.3}, {145/255:.3}, {155/255:.3}",
    f"{215/255:.3}, {200/255:.3}, {205/255:.3}",
    f"{255/255:.3}, {250/255:.3}, {250/255:.3}",
]


def gen_color():
    return choice(COLORS)


if __name__ == "__main__":
    print(
        """use crate::Vertex;

    pub const VERTICES: &[Vertex] = &["""
    )
    for idx1, idx2, idx3 in INDICES:
        color = gen_color()
        print(
            f"""    Vertex {{
            position: [{VERTICES[idx1][0]/500 - 1:.3}, {-VERTICES[idx1][1]/500 + 1:.3}, 0.0],
            color: [{color}],
        }},
        Vertex {{
            position: [{VERTICES[idx2][0]/500 - 1:.3}, {-VERTICES[idx2][1]/500 + 1:.3}, 0.0],
            color: [{color}],
        }},
        Vertex {{
            position: [{VERTICES[idx3][0]/500 - 1:.3}, {-VERTICES[idx3][1]/500 + 1:.3}, 0.0],
            color: [{color}],
        }},
    """
        )
    print("];")