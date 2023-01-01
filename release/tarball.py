import pathlib
import tarfile

from . import version


def main(args):
    """Create a tarball and print the path to it."""
    semver = version.get_version()
    repo_path = pathlib.Path(__file__).parent.parent
    sub_dir = f"python_launcher-{semver}"
    tar_contents = pathlib.Path(repo_path / sub_dir)

    binary_path = repo_path / "target" / args.target / "release" / "py"
    binary_tar = f"{sub_dir}/bin/py"

    license_path = repo_path / "LICENSE"
    license_tar = f"{sub_dir}/share/doc/py/LICENSE"

    readme_path = repo_path / "README.md"
    readme_tar = f"{sub_dir}/share/doc/py/README.md"

    man_path = repo_path / "man-page" / "py.1"
    man_tar = f"{sub_dir}/share/man/man1/py.1"

    fish_path = repo_path / "completions" / "py.fish"
    fish_tar = f"{sub_dir}/share/fish/vendor_completions.d/py.fish"

    tar_path = repo_path / f"{sub_dir}-{args.target}.tar.xz"
    with tarfile.open(tar_path, "w:xz") as tar_file:
        tar_file.add(binary_path, binary_tar)
        tar_file.add(license_path, license_tar)
        tar_file.add(readme_path, readme_tar)
        tar_file.add(man_path, man_tar)
        tar_file.add(fish_path, fish_tar)

    print(tar_path)
