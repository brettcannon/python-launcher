# Changelog fragments

This directory holds changelog fragments which list all various changes to be
recorded in the changelog upon release. They are managed by a project named
[scriv](https://scriv.readthedocs.io).

## Creating a fragment

### ... using `scriv`

`scriv create`

### ... manually

1. Create a file in the `changelog.d` directory named
   `<date>_<time>_<Git(Hub) username>.md` (e.g. `20210723_162141_brett.md` for
   an entry created on 2021-07-23 @ 16:21:41 local time by "brett").
1. Use the following as your file template, uncommenting the appropriate section
   for your entry.

```markdown
<!--
A new scriv changelog fragment.

Uncomment the section that is right (remove the HTML comment wrapper).
-->

<!--
### Removed

- A bullet item for the Removed category.

-->
<!--
### Added

- A bullet item for the Added category.

-->
<!--
### Changed

- A bullet item for the Changed category.

-->
<!--
### Deprecated

- A bullet item for the Deprecated category.

-->
<!--
### Fixed

- A bullet item for the Fixed category.

-->
<!--
### Security

- A bullet item for the Security category.

-->

```
