import pathlib

from scriv import scriv


def main(*args):
    """Create the release notes"""
    repo_path = pathlib.Path(__file__).parent.parent
    scriv_api = scriv.Scriv()
    changelog = scriv.Changelog(
        path=repo_path / "CHANGELOG.md", config=scriv_api.config
    )
    fragments = scriv_api.fragments_to_combine()
    section_dict = scriv_api.combine_fragments(fragments)
    release_notes = changelog.entry_text(section_dict)
    print(release_notes)
