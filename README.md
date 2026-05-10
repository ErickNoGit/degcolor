# Degcolor

Command-line interface (CLI) for ink management, designed for those who lack creativity or have trouble deciding which ink to choose. The features of this CLI are designed not only to display the paint color format to the user, but also to reveal the paint in the Linux shell.
# Commands and Features

`Degcolor` is a command-line interface (CLI) for color generation, visualization, and transformation directly inside the Linux terminal.

The tool supports multiple color formats such as `RGB`, `HEX`, and `HSL`, including utilities for inversion, color union, complementary colors, and terminal preview rendering.

---

## Command Reference

| Command | Description | Example |
|---|---|---|
| `degcolor --color` | Generates a random color with visual terminal context. | `degcolor --color` |
| `degcolor -c` | Short alias for `--color`. | `degcolor -c` |
| `degcolor --rgb` | Forces output in RGB format. | `degcolor --color --rgb` |
| `degcolor --hex` | Forces output in HEX format. | `degcolor --color --hex` |
| `degcolor --hsl` | Forces output in HSL format. | `degcolor --color --hsl` |
| `degcolor --context` | Removes visual context and returns only the raw color value. | `degcolor --color --hex --context` |
| `degcolor --reverse <COLOR>` | Returns the inverse/opposite color from the input. | `degcolor --reverse "#ff0000"` |
| `degcolor -r <COLOR>` | Short alias for `--reverse`. | `degcolor -r "rgb(255,0,0)"` |
| `degcolor --union <COLOR1> <COLOR2>` | Combines two colors into a new generated color. | `degcolor --union "#ff0000" "#0000ff"` |
| `degcolor -u <COLOR1> <COLOR2>` | Short alias for `--union`. | `degcolor -u "#ff0000" "#0000ff"` |
| `degcolor --magic <COLOR>` | Returns a nearest random inverse variation of the input color. | `degcolor --magic "#34a853"` |
| `degcolor -m <COLOR>` | Short alias for `--magic`. | `degcolor -m "rgb(10,20,30)"` |
| `degcolor --extreme <COLOR>` | Returns the complementary color of the input. | `degcolor --extreme "#ffffff"` |
| `degcolor -e <COLOR>` | Short alias for `--extreme`. | `degcolor -e "hsl(210, 50%, 40%)"` |
| `degcolor --show <COLOR>` | Displays the selected color directly in the terminal preview. | `degcolor --show "#ffaa00"` |
| `degcolor -s <COLOR>` | Short alias for `--show`. | `degcolor -s "rgb(255,170,0)"` |

---

## Examples

### Generate Random RGB Color

```bash
degcolor --color
```

Example output:

```text
+----+--------------------+
|    |  rgb(172, 91, 30)  |
+----+--------------------+
```

---

### Generate HEX Color Without Context

```bash
degcolor --color --hex --context
```

Output:

```text
#ac5b1e
```

---

### Show Color Preview

```bash
degcolor --show "#3498db"
```

---

### Combine Two Colors

```bash
degcolor --union "#ff0000" "#0000ff"
```

---

### Generate Complementary Color

```bash
degcolor --extreme "#ffffff"
```

---

## Features

- Random color generation
- RGB / HEX / HSL support
- Terminal color rendering
- Complementary color generation
- Color inversion
- Color union/mixing
- Lightweight CLI
- Linux shell integration
- Debian package installation
- Built entirely in Rust