import numpy as np
from matplotlib import path
from matplotlib.transforms import Bbox


def get_input() -> list[tuple[int, int]]:
    with open("day09/input.txt") as f:
        # false positive mypy error??
        return [tuple(map(int, line.split(",", maxsplit=1))) for line in f.read().splitlines()]


def main() -> int:
    """main"""
    path_vertices = get_input()
    coords_no = len(path_vertices)

    total = 0
    poly_v = np.asarray(path_vertices, int)
    poly = path.Path(poly_v, closed=True)

    # to_check = coords_no ** 2 // 2
    # checked = 0
    for x in range(coords_no):
        for y in range(x + 1, coords_no):
            # checked += 1
            # if checked % 10_000 == 0:
            #     print(f'Checked {checked} / {to_check} ({checked / to_check * 100:.2f} %) rectangle corners')
            corner1, corner2 = path_vertices[x], path_vertices[y]
            inner = Bbox([
                [min(corner1[0], corner2[0]), min(corner1[1], corner2[1])],
                [max(corner1[0], corner2[0]), max(corner1[1], corner2[1])],
            ])

            if poly.intersects_bbox(inner, filled=True) and not poly.intersects_bbox(inner, filled=False):
                # filled=False means False if no intersect, but the whole BBox is inside the polygon
                # thus, the difference of the two checks means the rectangle is fully inside the polygon
                total = max(total, (abs(corner2[0] - corner1[0]) + 1) * (abs(corner2[1] - corner1[1]) + 1))

    return total


if __name__ == '__main__':
    print(main())
