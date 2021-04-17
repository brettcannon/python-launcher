---
title: PY
section: 1
header: Python Launcher
footer: Python Launcher LAUNCHER_VERSION
date: CURRENT_DATE
---

# NAME

py - launch a Python interpreter

# SYNOPSIS

**py** [**-[X]/[X.Y]**] ...

# DESCRIPTION

**py** launches the most appropriate Python interpreter it can find. It is meant
to act as a shorthand for launching **python** without having to think about
_which_ Python interpreter is the most desired. The Python Launcher is not meant
to substitute all ways of launching Python, e.g. if a specific Python
interpreter is desired then it is assumed it will be directly executed.

# SPECIFYING A PYTHON VERSION

If a command-line option is provided in the form of **-X** or **-X.Y** where _X_
and _Y_ are integers, then that version of Python will be launched
(if available). For instance, providing **-3** will launch the newest version of
Python 3 while **-3.6** will try to launch Python 3.6.

# SEARCHING FOR PYTHON INTERPRETERS

When no command-line arguments are provided to the launcher, what is deemed the
most "appropriate" interpreter is searched for as follows:

1. An activated virtual environment (launched immediately if available)
2. A **.venv** directory in the current working directory or any parent
   directory containing a virtual environment
   (launched immediately if available)
3. If a file path is provided as the first argument, look for a shebang line
   containing **/usr/bin/python**, **/usr/local/bin/python**,
   **/usr/bin/env python** or **python** and any version specification in the
   executable name is treated as a version specifier (like with **-X**/**-X.Y**
   command-line options)
4. Check for any appropriate environment variable (see **ENVIRONMENT**)
5. Search **PATH** for all **pythonX.Y** executables
6. Launch the newest version of Python (while matching any version restrictions
   previously specified)

All unrecognized command-line arguments are passed on to the launched Python
interpreter.

# OPTIONS

**-h**/**--help**
: Print a help message and exit; must be specified on its own.

**--list**
: List all known interpreters (except activated virtual environment);
must be specified on its own.

**-[X]**
: Launch the latest Python _X_ version (e.g. **-3** for the latest
Python 3). See **ENVIRONMENT** for details on the **PY_VERSION[X]** environment
variable.

**-[X.Y]**
: Launch the specified Python version (e.g. **-3.6** for Python 3.6).

# ENVIRONMENT

**PY_PYTHON**
: Specify the version of Python to search for when no Python
version is explicitly requested (e.g. **3.6** to use Python 3.6 by
default).

**PY_PYTHON[X]**
: Specify the version of Python to search for when only a major
version is specified (e.g. set **PY_PYTHON3** to **3.6** to cause
**-3** to use Python 3.6).

**PYLAUNCH_DEBUG**
: Log details to stderr about how the Launcher is operating.

**VIRTUAL_ENV**
: Path to a directory containing virtual enviroment to use when no
Python version is explicitly requested; typically set by
activating a virtual environment.

**PATH**
: Used to search for Python interpreters.

# AUTHORS

Copyright © 2018 Brett Cannon.

Licensed under MIT.

# HOMEPAGE

https://github.com/brettcannon/python-launcher/

# SEE ALSO

python(1), python3(1).
