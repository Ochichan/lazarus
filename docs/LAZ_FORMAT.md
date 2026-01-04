# .laz Package Format Specification

**Version:** 1.0  
**Extension:** `.laz`  
**MIME Type:** `application/x-lazarus`

---

## Overview

The `.laz` format is a portable package format for sharing notes, flashcards, and educational content in offline environments. It's designed for:

- 📦 **USB-based distribution** in areas without internet
- 📚 **Educational content packaging** (textbooks, courses, curricula)
- 🔄 **Backup and migration** between Lazarus instances
- ✅ **Integrity verification** to detect corruption during transfer

---

## Format Structure

A `.laz` file is a **standard ZIP archive** with a specific internal structure:
```
Physics_101.laz (ZIP archive)
│
├── mimetype                    # MUST be first file, uncompressed
├── meta.json                   # Package metadata
├── manifest.json               # SHA-256 checksums for integrity
├── curriculum.json             # (Optional) Learning sequence
├── srs.json                    # (Optional) Flashcards
│
├── content/                    # Note content directory
│   ├── 550e8400-e29b-41d4-a716-446655440001.json
│   ├── 550e8400-e29b-41d4-a716-446655440002.json
│   └── 550e8400-e29b-41d4-a716-446655440003.json
│
└── assets/                     # (Optional) Media files
    ├── diagram_forces.png
    ├── audio_pronunciation.mp3
    └── video_experiment.mp4
```

---

## File Descriptions

### `mimetype`

**Purpose:** Identifies the file format (similar to EPUB/ODF standards)

**Requirements:**
- MUST be the first file in the ZIP
- MUST be stored uncompressed (STORE method)
- MUST contain exactly: `application/x-lazarus`
- No trailing newline

**Example:**
```
application/x-lazarus
```

---

### `meta.json`

**Purpose:** Package metadata for display and organization

**Schema:**
```json
{
  "title": "string (required)",
  "description": "string (optional)",
  "author": "string (optional)",
  "version": "string (optional, default: 1.0)",
  "created_at": "ISO 8601 datetime",
  "updated_at": "ISO 8601 datetime",
  "language": "ISO 639-1 code (optional)",
  "tags": ["array", "of", "strings"],
  "license": "string (optional)",
  "source_url": "string (optional)"
}
```

**Example:**
```json
{
  "title": "Classical Mechanics - Complete Course",
  "description": "Comprehensive introduction to Newtonian mechanics for first-year physics students. Covers kinematics, dynamics, energy, momentum, and rotational motion.",
  "author": "Dr. Jane Smith",
  "version": "2.1",
  "created_at": "2024-01-15T09:30:00Z",
  "updated_at": "2024-06-20T14:22:00Z",
  "language": "en",
  "tags": ["physics", "mechanics", "university", "stem"],
  "license": "CC BY-SA 4.0",
  "source_url": "https://example.edu/physics101"
}
```

---

### `manifest.json`

**Purpose:** SHA-256 checksums for integrity verification

**Why this matters:**
- USB drives can have bad sectors
- File transfers can be interrupted
- Storage media degrades over time
- Detects tampering or corruption

**Schema:**
```json
{
  "version": "1.0",
  "algorithm": "sha256",
  "files": {
    "relative/path/to/file": "sha256_hex_digest",
    ...
  }
}
```

**Example:**
```json
{
  "version": "1.0",
  "algorithm": "sha256",
  "files": {
    "meta.json": "a3f2b8c9d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1",
    "content/550e8400-e29b-41d4-a716-446655440001.json": "b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5",
    "content/550e8400-e29b-41d4-a716-446655440002.json": "c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6",
    "assets/diagram_forces.png": "d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7"
  }
}
```

---

### `curriculum.json`

**Purpose:** Define learning sequence and structure (optional)

**Use cases:**
- Online courses converted to offline
- Textbook chapter organization
- Guided learning paths

**Schema:**
```json
{
  "title": "string",
  "description": "string (optional)",
  "modules": [
    {
      "id": "string",
      "title": "string",
      "description": "string (optional)",
      "notes": ["uuid1", "uuid2", ...],
      "duration_minutes": "number (optional)",
      "prerequisites": ["module_id", ...] 
    }
  ]
}
```

**Example:**
```json
{
  "title": "Classical Mechanics - Complete Course",
  "description": "12-week introductory physics course",
  "modules": [
    {
      "id": "week-01",
      "title": "Week 1: Introduction to Kinematics",
      "description": "Position, velocity, acceleration in one dimension",
      "notes": [
        "550e8400-e29b-41d4-a716-446655440001",
        "550e8400-e29b-41d4-a716-446655440002"
      ],
      "duration_minutes": 180,
      "prerequisites": []
    },
    {
      "id": "week-02",
      "title": "Week 2: Vectors and 2D Motion",
      "description": "Vector algebra, projectile motion, circular motion",
      "notes": [
        "550e8400-e29b-41d4-a716-446655440003",
        "550e8400-e29b-41d4-a716-446655440004"
      ],
      "duration_minutes": 180,
      "prerequisites": ["week-01"]
    },
    {
      "id": "week-03",
      "title": "Week 3: Newton's Laws of Motion",
      "description": "Force, mass, acceleration, free body diagrams",
      "notes": [
        "550e8400-e29b-41d4-a716-446655440005",
        "550e8400-e29b-41d4-a716-446655440006",
        "550e8400-e29b-41d4-a716-446655440007"
      ],
      "duration_minutes": 240,
      "prerequisites": ["week-01", "week-02"]
    }
  ]
}
```

---

### `srs.json`

**Purpose:** Flashcards for spaced repetition learning (optional)

**Card Types:**
- `basic` - Simple Q&A
- `cloze` - Fill-in-the-blank
- `definition` - Term and definition

**Schema:**
```json
{
  "cards": [
    {
      "id": "uuid",
      "card_type": "basic|cloze|definition",
      "question": "string",
      "answer": "string",
      "note_id": "uuid (optional, links to source note)",
      "tags": ["array", "of", "strings"]
    }
  ]
}
```

**Example:**
```json
{
  "cards": [
    {
      "id": "card-001",
      "card_type": "basic",
      "question": "What is Newton's First Law of Motion?",
      "answer": "An object at rest stays at rest, and an object in motion stays in motion with the same speed and direction, unless acted upon by an external force.",
      "note_id": "550e8400-e29b-41d4-a716-446655440005",
      "tags": ["newton", "laws", "inertia"]
    },
    {
      "id": "card-002",
      "card_type": "cloze",
      "question": "F = {{m × a}}",
      "answer": "m × a",
      "note_id": "550e8400-e29b-41d4-a716-446655440005",
      "tags": ["newton", "formula"]
    },
    {
      "id": "card-003",
      "card_type": "definition",
      "question": "Momentum",
      "answer": "The product of an object's mass and velocity (p = mv). A vector quantity that represents the 'quantity of motion' of an object.",
      "note_id": "550e8400-e29b-41d4-a716-446655440008",
      "tags": ["momentum", "definition"]
    },
    {
      "id": "card-004",
      "card_type": "basic",
      "question": "What is the SI unit of force?",
      "answer": "Newton (N), where 1 N = 1 kg·m/s²",
      "note_id": "550e8400-e29b-41d4-a716-446655440005",
      "tags": ["units", "force"]
    }
  ]
}
```

---

### `content/*.json`

**Purpose:** Individual note files

**Filename:** UUID v4 with `.json` extension

**Why UUIDs?**
- Prevents filename collisions during import
- Enables cross-package references
- Simplifies merge operations

**Schema:**
```json
{
  "id": "uuid",
  "title": "string",
  "content": "string (markdown or plain text)",
  "tags": ["array", "of", "strings"],
  "created_at": "ISO 8601 datetime",
  "updated_at": "ISO 8601 datetime"
}
```

**Example (`content/550e8400-e29b-41d4-a716-446655440005.json`):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440005",
  "title": "Newton's Laws of Motion",
  "content": "# Newton's Laws of Motion\n\nSir Isaac Newton formulated three fundamental laws that describe the relationship between forces and motion.\n\n## First Law (Law of Inertia)\n\nAn object at rest stays at rest, and an object in motion stays in motion with the same speed and direction, unless acted upon by an external force.\n\n**Key concept:** Inertia is the tendency of an object to resist changes in its state of motion.\n\n## Second Law (F = ma)\n\nThe acceleration of an object is directly proportional to the net force acting on it and inversely proportional to its mass.\n\n$$F = ma$$\n\nWhere:\n- F = Force (Newtons)\n- m = Mass (kilograms)\n- a = Acceleration (m/s²)\n\n## Third Law (Action-Reaction)\n\nFor every action, there is an equal and opposite reaction.\n\nWhen object A exerts a force on object B, object B simultaneously exerts a force equal in magnitude but opposite in direction on object A.\n\n## Applications\n\n- **Rockets:** Thrust is generated by expelling gas (action), which pushes the rocket forward (reaction)\n- **Walking:** Your foot pushes backward on the ground, the ground pushes you forward\n- **Swimming:** Hands push water backward, water pushes swimmer forward",
  "tags": ["physics", "mechanics", "newton", "laws", "force"],
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

---

### `assets/`

**Purpose:** Media files referenced in notes (optional)

**Supported formats:**
- Images: `.png`, `.jpg`, `.jpeg`, `.gif`, `.webp`, `.svg`
- Audio: `.mp3`, `.ogg`, `.wav`
- Video: `.mp4`, `.webm`
- Documents: `.pdf`

**Storage method:** STORE (uncompressed) to avoid double-compression

**Referencing in notes:**
```markdown
![Force Diagram](assets/diagram_forces.png)

Listen to pronunciation: [audio](assets/audio_pronunciation.mp3)
```

---

## Complete Example Package

Here's a complete, real-world example of a `.laz` package:

### Package: `Physics_101_Week3.laz`

**Contents:**
```
Physics_101_Week3.laz
├── mimetype
├── meta.json
├── manifest.json
├── curriculum.json
├── srs.json
├── content/
│   ├── 550e8400-e29b-41d4-a716-446655440005.json  (Newton's Laws)
│   ├── 550e8400-e29b-41d4-a716-446655440006.json  (Free Body Diagrams)
│   └── 550e8400-e29b-41d4-a716-446655440007.json  (Practice Problems)
└── assets/
    ├── fbd_example1.png
    ├── fbd_example2.png
    └── friction_demo.mp4
```

**`mimetype`:**
```
application/x-lazarus
```

**`meta.json`:**
```json
{
  "title": "Physics 101 - Week 3: Newton's Laws",
  "description": "Introduction to Newton's three laws of motion with worked examples and practice problems.",
  "author": "Physics Department",
  "version": "1.0",
  "created_at": "2024-09-01T08:00:00Z",
  "updated_at": "2024-09-01T08:00:00Z",
  "language": "en",
  "tags": ["physics", "mechanics", "newton", "university", "week3"],
  "license": "CC BY-NC-SA 4.0"
}
```

**`curriculum.json`:**
```json
{
  "title": "Week 3: Newton's Laws of Motion",
  "description": "Master the fundamental laws governing force and motion",
  "modules": [
    {
      "id": "theory",
      "title": "Theory: Newton's Three Laws",
      "notes": ["550e8400-e29b-41d4-a716-446655440005"],
      "duration_minutes": 45
    },
    {
      "id": "diagrams",
      "title": "Skill: Free Body Diagrams",
      "notes": ["550e8400-e29b-41d4-a716-446655440006"],
      "duration_minutes": 60,
      "prerequisites": ["theory"]
    },
    {
      "id": "practice",
      "title": "Practice: Problem Set",
      "notes": ["550e8400-e29b-41d4-a716-446655440007"],
      "duration_minutes": 90,
      "prerequisites": ["theory", "diagrams"]
    }
  ]
}
```

**`srs.json`:**
```json
{
  "cards": [
    {
      "id": "card-w3-001",
      "card_type": "basic",
      "question": "State Newton's First Law",
      "answer": "An object at rest stays at rest, and an object in motion stays in motion at constant velocity, unless acted upon by a net external force.",
      "note_id": "550e8400-e29b-41d4-a716-446655440005",
      "tags": ["newton", "first-law"]
    },
    {
      "id": "card-w3-002",
      "card_type": "cloze",
      "question": "Newton's Second Law: F = {{ma}}",
      "answer": "ma",
      "note_id": "550e8400-e29b-41d4-a716-446655440005",
      "tags": ["newton", "second-law", "formula"]
    },
    {
      "id": "card-w3-003",
      "card_type": "basic",
      "question": "What is inertia?",
      "answer": "The tendency of an object to resist changes in its state of motion. Objects with more mass have more inertia.",
      "note_id": "550e8400-e29b-41d4-a716-446655440005",
      "tags": ["inertia", "definition"]
    },
    {
      "id": "card-w3-004",
      "card_type": "basic",
      "question": "A 5 kg object experiences a net force of 20 N. What is its acceleration?",
      "answer": "a = F/m = 20 N / 5 kg = 4 m/s²",
      "note_id": "550e8400-e29b-41d4-a716-446655440007",
      "tags": ["newton", "second-law", "calculation"]
    },
    {
      "id": "card-w3-005",
      "card_type": "definition",
      "question": "Free Body Diagram",
      "answer": "A simplified drawing showing all forces acting on an object, represented as arrows pointing in the direction of each force.",
      "note_id": "550e8400-e29b-41d4-a716-446655440006",
      "tags": ["fbd", "definition"]
    }
  ]
}
```

**`content/550e8400-e29b-41d4-a716-446655440006.json`:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440006",
  "title": "Free Body Diagrams",
  "content": "# Free Body Diagrams (FBD)\n\nA Free Body Diagram is a visual representation of all forces acting on an object.\n\n## Steps to Draw an FBD\n\n1. **Identify the object** - Draw it as a simple shape (box, dot, or outline)\n2. **Identify all forces** - List every force acting ON the object\n3. **Draw force arrows** - Start from center, point in direction of force\n4. **Label each force** - Include magnitude if known\n\n## Common Forces\n\n| Force | Symbol | Direction |\n|-------|--------|----------|\n| Weight | W or Fg | Downward (toward Earth) |\n| Normal | N or Fn | Perpendicular to surface |\n| Friction | f | Opposite to motion/tendency |\n| Tension | T | Along rope/string |\n| Applied | Fa | Direction of push/pull |\n\n## Example 1: Book on a Table\n\n![Book on table FBD](assets/fbd_example1.png)\n\nForces:\n- Weight (W) pointing down\n- Normal force (N) pointing up\n- Since book is stationary: N = W\n\n## Example 2: Block on Inclined Plane\n\n![Block on incline FBD](assets/fbd_example2.png)\n\nForces:\n- Weight (W) pointing straight down\n- Normal force (N) perpendicular to surface\n- Friction (f) parallel to surface, opposing motion\n\n## Tips\n\n- Only draw forces acting ON the object, not forces the object exerts\n- Don't include velocity or acceleration arrows in FBD\n- Choose a convenient coordinate system (often aligned with motion)",
  "tags": ["physics", "mechanics", "fbd", "diagrams", "problem-solving"],
  "created_at": "2024-09-01T08:30:00Z",
  "updated_at": "2024-09-01T08:30:00Z"
}
```

**`manifest.json`:**
```json
{
  "version": "1.0",
  "algorithm": "sha256",
  "files": {
    "meta.json": "8a7b6c5d4e3f2a1b0c9d8e7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b",
    "curriculum.json": "1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b",
    "srs.json": "2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3",
    "content/550e8400-e29b-41d4-a716-446655440005.json": "3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4",
    "content/550e8400-e29b-41d4-a716-446655440006.json": "4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5",
    "content/550e8400-e29b-41d4-a716-446655440007.json": "5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6",
    "assets/fbd_example1.png": "6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7",
    "assets/fbd_example2.png": "7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8",
    "assets/friction_demo.mp4": "8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9"
  }
}
```

---

## Creating a .laz Package

### Using Lazarus UI

1. Go to **Notes** page
2. Select notes to export (checkbox)
3. Click **Export**
4. Enter title, author, description
5. Click **Create Package**
6. Download `.laz` file

### Programmatically (Bash)
```bash
#!/bin/bash
# create_laz.sh - Create a .laz package from directory

PACKAGE_NAME="$1"
SOURCE_DIR="$2"

# Create temporary directory
TEMP_DIR=$(mktemp -d)
cp -r "$SOURCE_DIR"/* "$TEMP_DIR/"

# Create mimetype (must be first, uncompressed)
echo -n "application/x-lazarus" > "$TEMP_DIR/mimetype"

# Create manifest with checksums
cd "$TEMP_DIR"
echo '{"version":"1.0","algorithm":"sha256","files":{' > manifest.json
first=true
for file in $(find . -type f ! -name mimetype ! -name manifest.json | sort); do
    file="${file#./}"
    hash=$(sha256sum "$file" | cut -d' ' -f1)
    if [ "$first" = true ]; then
        first=false
    else
        echo "," >> manifest.json
    fi
    echo -n "\"$file\":\"$hash\"" >> manifest.json
done
echo '}}' >> manifest.json

# Create ZIP (mimetype first, uncompressed)
cd "$TEMP_DIR"
zip -0 -X "../$PACKAGE_NAME.laz" mimetype
zip -r -X "../$PACKAGE_NAME.laz" . -x mimetype

# Cleanup
rm -rf "$TEMP_DIR"

echo "Created: $PACKAGE_NAME.laz"
```

---

## Importing a .laz Package

### Using Lazarus UI

1. Go to **Notes** page
2. Click **Import** or drag-and-drop `.laz` file
3. Review import summary
4. Confirm import

### Verification Process

When importing, Lazarus:

1. Validates `mimetype` file
2. Parses `manifest.json`
3. Verifies SHA-256 checksums for all files
4. Reports any corrupted files
5. Imports valid content
6. Handles duplicate notes (keeps newest)

---

## Design Decisions

### Why ZIP-based?

- Universal compatibility (any OS can open)
- Well-tested compression
- Streaming support
- No custom tooling needed for inspection

### Why UUIDs for filenames?

- Collision-free across packages
- Enables merge operations
- Supports cross-references
- No filename encoding issues

### Why SHA-256 manifest?

- Detects bit rot on storage media
- Catches incomplete transfers
- Verifies integrity without external tools
- Industry-standard algorithm

### Why uncompressed assets?

- Images/videos already compressed
- Avoids CPU overhead for decompression
- Faster random access
- Prevents quality loss from re-compression

---

## Compatibility

The `.laz` format is designed for forward compatibility:

- Unknown JSON fields are ignored (not errors)
- Missing optional files are handled gracefully
- New features can be added without breaking old readers

**Minimum required files:**
- `mimetype`
- `meta.json`
- At least one file in `content/`

---

## Tools

### Inspect a .laz file
```bash
# List contents
unzip -l package.laz

# Extract
unzip package.laz -d extracted/

# View metadata
unzip -p package.laz meta.json | jq .

# Verify checksums manually
unzip -p package.laz manifest.json | jq -r '.files | to_entries[] | "\(.value)  \(.key)"' | sha256sum -c
```

### Validate structure
```bash
# Check mimetype
unzip -p package.laz mimetype
# Should output: application/x-lazarus

# Check required files exist
unzip -l package.laz | grep -E "mimetype|meta.json|content/"
```

---

## Future Extensions (Planned)

- **Encryption:** Package-level encryption for sensitive content
- **Signatures:** Author signatures for authenticity
- **Deltas:** Incremental updates to reduce transfer size
- **Dependencies:** Reference other packages

---

*The .laz format: Simple, portable, verifiable.*
