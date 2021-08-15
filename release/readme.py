import pathlib


def main(args):
    repo_path = pathlib.Path(__file__).parent.parent
    template_path = repo_path / "docs" / "readme" / "template.md"
    template = template_path.read_text(encoding="utf-8")
    new_readme = template.format(version=args.version, tag=args.tag)
    readme_path = repo_path / "README.md"
    readme_path.write_text(new_readme, encoding="utf-8")
