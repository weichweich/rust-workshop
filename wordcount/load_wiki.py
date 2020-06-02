import sys
import pathlib


try:
    import wikipedia
except Exception as e:
    raise RuntimeError("install wikipedia `pip3 install wikipedia`") from e


def main(out: str):
    outpath = pathlib.Path(out)

    random_pages = map(lambda title: wikipedia.page(title), wikipedia.random(10))
    for page in random_pages:
        filename = outpath.joinpath(page.title).with_suffix(".txt")
        with filename.open("w") as f:
            f.write(page.content)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("expected output path!")
    else:
        main(sys.argv[1])
