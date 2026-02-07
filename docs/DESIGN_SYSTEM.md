# Maestro OS — Design System v1.0

**Document Version:** 1.0  
**Last Updated:** February 7, 2026  
**Applies To:** Maestro OS Desktop Environment, Web Demo, Marketing Materials

---

## Table of Contents

1. [Design Philosophy](#1-design-philosophy)
2. [Color System](#2-color-system)
3. [Typography](#3-typography)
4. [Spacing & Layout](#4-spacing--layout)
5. [Ternary State System](#5-ternary-state-system)
6. [Component Specifications](#6-component-specifications)
7. [Iconography](#7-iconography)
8. [Animation & Motion](#8-animation--motion)
9. [Security Mode Theming](#9-security-mode-theming)
10. [Accessibility](#10-accessibility)
11. [Implementation Notes](#11-implementation-notes)

---

## 1. Design Philosophy

### Core Principles

1. **Ternary-First** — Every interactive element supports three states (-1, 0, +1), not binary
2. **Dark by Default** — Optimized for extended use; reduces eye strain
3. **Information Density** — Professional users need data, not whitespace
4. **Quantum Aesthetic** — Subtle glow effects, phase animations, teal accents
5. **Embossed Depth** — UI panels use light/shadow to create tactile depth

### Visual Identity

Maestro OS combines:
- **Post-quantum gravitas** — Serious, secure, professional
- **Ternary innovation** — Unique three-state interactions
- **Premium polish** — Refined shadows, smooth animations, careful typography

---

## 2. Color System

### 2.1 Core Palette

```
┌─────────────────────────────────────────────────────────────────┐
│  BACKGROUND HIERARCHY                                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Deep Void      #050709    rgb(5, 7, 9)       ← Deepest       │
│  Background     #080A0F    rgb(8, 10, 15)     ← Primary BG    │
│  Surface        #0A0D14    rgb(10, 13, 20)    ← Cards         │
│  Elevated       #0F1219    rgb(15, 18, 25)    ← Modals        │
│  Sidebar        #0C0F16    rgb(12, 15, 22)    ← Navigation    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  PRIMARY: QUANTUM TEAL                                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Teal 900       #065F46    rgb(6, 95, 70)     ← Darkest       │
│  Teal 700       #087B5E    rgb(8, 123, 94)                     │
│  Teal 500       #0F9D7A    rgb(15, 157, 122)  ← PRIMARY       │
│  Teal 400       #14B892    rgb(20, 184, 146)                   │
│  Teal 300       #00FFAA    rgb(0, 255, 170)   ← Glow/Accent   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  TERNARY STATE COLORS                                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  State -1       #EF4444    rgb(239, 68, 68)   ← Red           │
│  State -1 Dark  #DC2626    rgb(220, 38, 38)                    │
│  State -1 Muted #7F1D1D    rgb(127, 29, 29)                    │
│                                                                 │
│  State 0        #6B7280    rgb(107, 114, 128) ← Gray          │
│  State 0 Light  #9CA3AF    rgb(156, 163, 175)                  │
│  State 0 Dark   #4B5563    rgb(75, 85, 99)                     │
│                                                                 │
│  State +1       #22C55E    rgb(34, 197, 94)   ← Green         │
│  State +1 Dark  #16A34A    rgb(22, 163, 74)                    │
│  State +1 Muted #14532D    rgb(20, 83, 45)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  TEXT COLORS                                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Primary        #F9FAFB    rgb(249, 250, 251) ← Headings      │
│  Secondary      #E5E7EB    rgb(229, 231, 235) ← Body          │
│  Muted          #9CA3AF    rgb(156, 163, 175) ← Captions      │
│  Disabled       #6B7280    rgb(107, 114, 128)                  │
│  Inverse        #111827    rgb(17, 24, 39)    ← On light BG   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  BORDER & DIVIDER                                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Border Default #1F2937    rgb(31, 41, 55)                     │
│  Border Subtle  #1A1D26    rgb(26, 29, 38)                     │
│  Border Focus   #0F9D7A    rgb(15, 157, 122)  ← Teal          │
│  Divider        #374151    rgb(55, 65, 81)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Security Mode Colors

| Mode | Primary | Background | Border | Usage |
|------|---------|------------|--------|-------|
| **Mode 0** (Hypervisor) | `#EF4444` | `rgba(239,68,68,0.1)` | `#DC2626` | Highest privilege |
| **Mode 1** (Kernel) | `#F59E0B` | `rgba(245,158,11,0.1)` | `#D97706` | Kernel operations |
| **Mode φ** (Supervisor) | `#FBBF24` | `rgba(251,191,36,0.1)` | `#F59E0B` | System services |
| **Mode φ+** (User) | `#22C55E` | `rgba(34,197,94,0.1)` | `#16A34A` | User applications |

### 2.3 Semantic Colors

```
Success:    #22C55E (Green)
Warning:    #F59E0B (Amber)
Error:      #EF4444 (Red)
Info:       #3B82F6 (Blue)
```

### 2.4 CSS Custom Properties

```css
:root {
  /* Backgrounds */
  --color-bg-deep: #050709;
  --color-bg-primary: #080A0F;
  --color-bg-surface: #0A0D14;
  --color-bg-elevated: #0F1219;
  --color-bg-sidebar: #0C0F16;
  
  /* Primary */
  --color-primary-900: #065F46;
  --color-primary-700: #087B5E;
  --color-primary-500: #0F9D7A;
  --color-primary-400: #14B892;
  --color-primary-300: #00FFAA;
  
  /* Ternary States */
  --color-trit-negative: #EF4444;
  --color-trit-zero: #6B7280;
  --color-trit-positive: #22C55E;
  
  /* Text */
  --color-text-primary: #F9FAFB;
  --color-text-secondary: #E5E7EB;
  --color-text-muted: #9CA3AF;
  --color-text-disabled: #6B7280;
  
  /* Borders */
  --color-border-default: #1F2937;
  --color-border-subtle: #1A1D26;
  --color-border-focus: #0F9D7A;
  
  /* Glow */
  --color-glow-primary: #00FFAA;
  --glow-radius-sm: 4px;
  --glow-radius-md: 10px;
  --glow-radius-lg: 20px;
}
```

---

## 3. Typography

### 3.1 Font Families

| Purpose | Font Stack | Fallbacks |
|---------|------------|-----------|
| **Display/Headers** | Inter | system-ui, -apple-system, sans-serif |
| **Body** | Inter | system-ui, -apple-system, sans-serif |
| **Monospace/Code** | JetBrains Mono | Fira Code, Consolas, monospace |
| **Document Headers** | Felix Titling* | Georgia, serif |
| **Document Body** | Century Gothic* | Helvetica Neue, sans-serif |

*Felix Titling and Century Gothic are used for formal documents only (PDF exports, official materials).

### 3.2 Type Scale

```
┌─────────────────────────────────────────────────────────────────┐
│  TYPE SCALE (Based on 16px root)                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  xs      0.75rem    12px     Leading: 1.0rem    (16px)        │
│  sm      0.875rem   14px     Leading: 1.25rem   (20px)        │
│  base    1rem       16px     Leading: 1.5rem    (24px)        │
│  lg      1.125rem   18px     Leading: 1.75rem   (28px)        │
│  xl      1.25rem    20px     Leading: 1.75rem   (28px)        │
│  2xl     1.5rem     24px     Leading: 2rem      (32px)        │
│  3xl     1.875rem   30px     Leading: 2.25rem   (36px)        │
│  4xl     2.25rem    36px     Leading: 2.5rem    (40px)        │
│  5xl     3rem       48px     Leading: 1          (48px)        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 Font Weights

| Weight | Value | Usage |
|--------|-------|-------|
| Regular | 400 | Body text, descriptions |
| Medium | 500 | Buttons, labels |
| Semi-bold | 600 | Section headers, emphasis |
| Bold | 700 | Page titles, important headings |

### 3.4 Letter Spacing

| Context | Tracking | CSS Value |
|---------|----------|-----------|
| All Caps Labels | Wide | `0.05em` |
| Page Titles | Tight | `-0.01em` |
| Body Text | Normal | `0` |
| Code | Normal | `0` |

### 3.5 Ternary State Typography

Typography scales with ternary state:

| Property | State -1 | State 0 | State +1 |
|----------|----------|---------|----------|
| Base Size | 14px | 16px | 18px |
| Scaling Factor | 0.875 | 1.0 | 1.125 |
| Letter Spacing | +0.02em | +0.01em | 0 |
| Weight Adjustment | -100 | 0 | +100 |

---

## 4. Spacing & Layout

### 4.1 Spacing Scale

```
┌─────────────────────────────────────────────────────────────────┐
│  SPACING SCALE (4px base unit)                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  0       0px        --space-0                                  │
│  px      1px        --space-px                                 │
│  0.5     2px        --space-0.5                                │
│  1       4px        --space-1                                  │
│  1.5     6px        --space-1.5                                │
│  2       8px        --space-2                                  │
│  2.5     10px       --space-2.5                                │
│  3       12px       --space-3                                  │
│  4       16px       --space-4                                  │
│  5       20px       --space-5                                  │
│  6       24px       --space-6                                  │
│  8       32px       --space-8                                  │
│  10      40px       --space-10                                 │
│  12      48px       --space-12                                 │
│  16      64px       --space-16                                 │
│  20      80px       --space-20                                 │
│  24      96px       --space-24                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 Layout Constants

| Element | Dimension |
|---------|-----------|
| Sidebar Width (Collapsed) | 64px |
| Sidebar Width (Neutral) | 240px |
| Sidebar Width (Expanded) | 320px |
| Top Bar Height | 40px |
| Card Border Radius | 8px |
| Button Border Radius | 6px |
| Input Border Radius | 6px |
| Modal Border Radius | 12px |
| Icon Size (sm) | 16px |
| Icon Size (md) | 20px |
| Icon Size (lg) | 24px |

### 4.3 Grid System

- **Container Max Width:** 1440px
- **Gutter Width:** 24px (desktop), 16px (tablet), 12px (mobile)
- **Columns:** 12-column grid
- **Breakpoints:**
  - `sm`: 640px
  - `md`: 768px
  - `lg`: 1024px
  - `xl`: 1280px
  - `2xl`: 1536px

---

## 5. Ternary State System

### 5.1 State Definitions

Every interactive element supports three states:

| State | Value | Meaning | Visual Characteristics |
|-------|-------|---------|------------------------|
| **Minimized** | -1 | Collapsed, reduced, background | Smaller, muted colors, less detail |
| **Neutral** | 0 | Default, standard, balanced | Normal appearance |
| **Expanded** | +1 | Full detail, enhanced, focused | Larger, brighter, more information |

### 5.2 State Transitions

```
      ┌──────────────────────────────────────────┐
      │                                          │
      │   -1 ◄────────► 0 ◄────────► +1         │
      │   Minimized    Neutral     Expanded      │
      │                                          │
      │   ◄── 100ms ──► ◄── 200ms ──►           │
      │    (collapse)    (expand)                │
      │                                          │
      └──────────────────────────────────────────┘
```

| Transition | Duration | Easing |
|------------|----------|--------|
| -1 → 0 (Expand from min) | 150ms | ease-out |
| 0 → -1 (Collapse) | 100ms | ease-out |
| 0 → +1 (Expand) | 250ms | ease-in-out |
| +1 → 0 (Contract) | 200ms | ease-out |

### 5.3 Visual Mapping

| Property | State -1 | State 0 | State +1 |
|----------|----------|---------|----------|
| **Opacity** | 0.7 | 1.0 | 1.0 |
| **Shadow Intensity** | 0.2 | 0.4 | 0.6 |
| **Glow Radius** | 0px | 10px | 25px |
| **Glow Opacity** | 0 | 0.3 | 0.6 |
| **Border Width** | 0.5px | 1px | 2px |
| **Animation Speed** | 0.5x | 1x | 1.5x |

---

## 6. Component Specifications

### 6.1 Sidebar

```
┌──────────────────────────────────────────────────────┐
│  SIDEBAR ANATOMY                                     │
├──────────────────────────────────────────────────────┤
│                                                      │
│  ┌──────────────────────────────────┐               │
│  │  EMBOSSED TOP FRAME              │  height: 100px│
│  │  ┌────────────────────────────┐  │               │
│  │  │ ◇ MAESTRO OS              │  │               │
│  │  │   Post-Quantum Desktop     │  │               │
│  │  │ ≋≋≋≋≋≋ 94% ≋≋≋≋≋≋        │  │  Quantum Wave │
│  │  └────────────────────────────┘  │               │
│  └──────────────────────────────────┘               │
│                                      shadow: inner  │
│  ┌──────────────────────────────────┐               │
│  │  NAVIGATION SECTION              │               │
│  │                                  │               │
│  │  📊 Dashboard              ●     │  Active dot   │
│  │  📁 Projects              (12)   │  Badge        │
│  │  👥 People                       │               │
│  │  💰 Finance                      │               │
│  │  📄 Documents            (47)    │               │
│  │  🧠 Intelligence                 │               │
│  │                                  │               │
│  └──────────────────────────────────┘               │
│                                                      │
│  ┌──────────────────────────────────┐               │
│  │  RECESSED BOTTOM FRAME           │  height: 120px│
│  │                                  │               │
│  │  🔒 Security                     │               │
│  │  ⚙️ Settings                     │               │
│  │  ─────────────────────────────   │  Divider      │
│  │  ┌────────────────────────────┐  │               │
│  │  │ 👤 │ Salvi                 │  │  User Card    │
│  │  │    │ Admin • Mode φ+       │  │               │
│  │  └────────────────────────────┘  │               │
│  └──────────────────────────────────┘               │
│                                      shadow: inner  │
└──────────────────────────────────────────────────────┘
```

**Embossed Frame CSS:**
```css
.embossed-frame {
  background: linear-gradient(180deg, #0F1219 0%, #0A0D14 100%);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.05),
    inset 0 -1px 0 rgba(0, 0, 0, 0.3),
    0 4px 12px rgba(0, 0, 0, 0.4);
}

.recessed-frame {
  background: linear-gradient(180deg, #080A0F 0%, #0A0D14 100%);
  box-shadow:
    inset 0 2px 4px rgba(0, 0, 0, 0.4),
    inset 0 -1px 0 rgba(255, 255, 255, 0.03);
}
```

### 6.2 TernarySwitch

The signature three-state toggle:

```
┌─────────────────────────────────────────────────────┐
│  TERNARY SWITCH                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Normal State:                                      │
│  ┌─────────┬─────────┬─────────┐                   │
│  │   -1    │    0    │   +1    │  height: 32px    │
│  │  ░░░░░  │  █████  │  ░░░░░  │  width: 96px     │
│  │         │    ●    │         │  ← thumb (12px)  │
│  └─────────┴─────────┴─────────┘                   │
│     Red      Gray      Green                        │
│                                                     │
│  Active Positions:                                  │
│  ● at x=16px   → State -1                          │
│  ● at x=48px   → State 0                           │
│  ● at x=80px   → State +1                          │
│                                                     │
│  Interaction:                                       │
│  - Click region selects state                       │
│  - Drag thumb between positions                     │
│  - Keyboard: ←/→ arrows cycle states               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 6.3 Security Mode Badge

```
┌─────────────────────────────────────────────────────┐
│  SECURITY BADGE VARIANTS                            │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Mode 0 (Hypervisor):                               │
│  ┌───────────────────────┐                         │
│  │ 🛡️ MODE 0 │ Hypervisor │  bg: rgba(red, 0.1)   │
│  └───────────────────────┘   border: #DC2626       │
│                                                     │
│  Mode 1 (Kernel):                                   │
│  ┌───────────────────────┐                         │
│  │ 🛡️ MODE 1 │ Kernel    │  bg: rgba(orange, 0.1) │
│  └───────────────────────┘   border: #D97706       │
│                                                     │
│  Mode φ (Supervisor):                               │
│  ┌───────────────────────┐                         │
│  │ 🛡️ MODE φ │ Supervisor │  bg: rgba(yellow, 0.1)│
│  └───────────────────────┘   border: #F59E0B       │
│                                                     │
│  Mode φ+ (User) - Active:                          │
│  ┌───────────────────────┐                         │
│  │ 🛡️ MODE φ+│ User      │  bg: rgba(green, 0.15) │
│  └───────────────────────┘   border: #16A34A       │
│                              + glow effect          │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 6.4 Quantum Wave Indicator

```
┌─────────────────────────────────────────────────────┐
│  QUANTUM WAVE / COHERENCE INDICATOR                 │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Minimized (State -1):                              │
│  ─────────────────────  (static line)              │
│                                                     │
│  Neutral (State 0):                                 │
│  ~~~~~≋~~~~~≋~~~~~   (gentle sine wave)            │
│                                                     │
│  Expanded (State +1):                               │
│  ≋≋≋≋≋≋≋≋≋≋≋ 94%   (active wave + percentage)     │
│                                                     │
│  Properties:                                        │
│  - Amplitude: 2px (min) → 6px (max)                │
│  - Frequency: 1Hz (min) → 3Hz (max)                │
│  - Color: Quantum Teal (#00FFAA)                   │
│  - Glow: Increases with coherence level            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 6.5 Statistics Card

```
┌─────────────────────────────────────────────────────┐
│  STAT CARD                                          │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────────────────────────┐               │
│  │ 📊                    [Badge]   │  icon + badge │
│  │                                 │               │
│  │ Projects                        │  title (muted)│
│  │ 12                              │  value (bold) │
│  │                                 │               │
│  │ ▲ 3 active                      │  trend/detail │
│  └─────────────────────────────────┘               │
│                                                     │
│  Size: 180px × 120px (default)                     │
│  Padding: 16px                                      │
│  Border Radius: 8px                                 │
│  Background: var(--color-bg-surface)               │
│  Border: 1px solid var(--color-border-subtle)      │
│                                                     │
│  Hover Effect:                                      │
│  - translateY(-2px)                                │
│  - box-shadow intensifies                          │
│  - border gains teal tint                          │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 6.6 Button Variants

| Variant | Background | Border | Text | Hover |
|---------|------------|--------|------|-------|
| **Primary** | `#0F9D7A` | none | white | brighten 10% |
| **Secondary** | transparent | `#374151` | `#E5E7EB` | bg: `#1F2937` |
| **Ghost** | transparent | none | `#9CA3AF` | bg: `#1F2937` |
| **Destructive** | `#DC2626` | none | white | brighten 10% |
| **Outline** | transparent | `#0F9D7A` | `#0F9D7A` | bg: `rgba(15,157,122,0.1)` |

---

## 7. Iconography

### 7.1 Icon Style

- **Library:** Lucide React (primary), custom SVG (specialized)
- **Stroke Width:** 1.5px (default), 2px (emphasis)
- **Style:** Outlined, rounded corners
- **Sizes:** 16px (sm), 20px (md), 24px (lg), 32px (xl)

### 7.2 Navigation Icons

| Section | Icon | Lucide Name |
|---------|------|-------------|
| Dashboard | 📊 | `LayoutDashboard` |
| Projects | 📁 | `FolderKanban` |
| People | 👥 | `Users` |
| Finance | 💰 | `DollarSign` |
| Documents | 📄 | `Files` |
| Intelligence | 🧠 | `Brain` |
| Security | 🔒 | `Shield` |
| Settings | ⚙️ | `Settings` |

### 7.3 Ternary-Specific Icons

Custom SVG icons for:
- Trit indicator (`TritIcon`)
- Phase rotation (`PhaseIcon`)
- Torus network (`TorusIcon`)
- Quantum state (`QuantumIcon`)
- 13D tag (`DimensionIcon`)

---

## 8. Animation & Motion

### 8.1 Timing Functions

| Name | CSS Value | Usage |
|------|-----------|-------|
| **ease-default** | `cubic-bezier(0.4, 0, 0.2, 1)` | General transitions |
| **ease-in** | `cubic-bezier(0.4, 0, 1, 1)` | Enter animations |
| **ease-out** | `cubic-bezier(0, 0, 0.2, 1)` | Exit animations |
| **ease-bounce** | `cubic-bezier(0.68, -0.55, 0.265, 1.55)` | Playful interactions |
| **ease-spring** | `cubic-bezier(0.34, 1.56, 0.64, 1)` | Ternary switch |

### 8.2 Duration Scale

| Token | Duration | Usage |
|-------|----------|-------|
| `instant` | 0ms | Immediate feedback |
| `fast` | 100ms | Hover states, toggles |
| `normal` | 200ms | Most transitions |
| `slow` | 300ms | Complex animations |
| `slower` | 500ms | Page transitions |

### 8.3 Standard Animations

```css
/* Fade In */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* Slide Up */
@keyframes slideUp {
  from { transform: translateY(10px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* Pulse Glow */
@keyframes pulseGlow {
  0%, 100% { opacity: 0.5; box-shadow: 0 0 10px var(--color-glow-primary); }
  50% { opacity: 1; box-shadow: 0 0 20px var(--color-glow-primary); }
}

/* Quantum Wave */
@keyframes quantumWave {
  0% { transform: translateX(0); }
  100% { transform: translateX(-50%); }
}

/* Rotate (for loading) */
@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
```

---

## 9. Security Mode Theming

Each security mode subtly adjusts the UI:

### 9.1 Mode 0 (Hypervisor)

```css
[data-security-mode="0"] {
  --accent-color: #EF4444;
  --accent-glow: rgba(239, 68, 68, 0.3);
  --border-accent: #DC2626;
}
```

### 9.2 Mode 1 (Kernel)

```css
[data-security-mode="1"] {
  --accent-color: #F59E0B;
  --accent-glow: rgba(245, 158, 11, 0.3);
  --border-accent: #D97706;
}
```

### 9.3 Mode φ (Supervisor)

```css
[data-security-mode="phi"] {
  --accent-color: #FBBF24;
  --accent-glow: rgba(251, 191, 36, 0.3);
  --border-accent: #F59E0B;
}
```

### 9.4 Mode φ+ (User) — Default

```css
[data-security-mode="phi-plus"] {
  --accent-color: #22C55E;
  --accent-glow: rgba(34, 197, 94, 0.3);
  --border-accent: #16A34A;
}
```

---

## 10. Accessibility

### 10.1 Color Contrast

All text meets WCAG 2.1 AA standards:

| Combination | Contrast Ratio | Status |
|-------------|----------------|--------|
| Primary text on background | 15.8:1 | ✅ AAA |
| Secondary text on background | 12.1:1 | ✅ AAA |
| Muted text on background | 7.2:1 | ✅ AA |
| Primary teal on background | 6.4:1 | ✅ AA |

### 10.2 Focus States

All interactive elements have visible focus indicators:

```css
:focus-visible {
  outline: 2px solid var(--color-primary-500);
  outline-offset: 2px;
}
```

### 10.3 Motion Preferences

Respect user's reduced motion preference:

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

### 10.4 Screen Reader Support

- All icons have `aria-label` or accompanying text
- Ternary states announced: "State: Minimized/Neutral/Expanded"
- Security modes announced: "Security Mode: Phi Plus, User Level"

---

## 11. Implementation Notes

### 11.1 Web Demo (React/TypeScript)

```typescript
// Tailwind CSS config extension
module.exports = {
  theme: {
    extend: {
      colors: {
        background: '#080A0F',
        surface: '#0A0D14',
        primary: {
          500: '#0F9D7A',
          300: '#00FFAA',
        },
        trit: {
          negative: '#EF4444',
          zero: '#6B7280',
          positive: '#22C55E',
        },
      },
    },
  },
};
```

### 11.2 Native OS (Rust)

```rust
// Theme constants
pub mod theme {
    pub const COLOR_BG_PRIMARY: u32 = 0xFF080A0F;
    pub const COLOR_BG_SURFACE: u32 = 0xFF0A0D14;
    pub const COLOR_PRIMARY_500: u32 = 0xFF0F9D7A;
    pub const COLOR_PRIMARY_GLOW: u32 = 0xFF00FFAA;
    pub const COLOR_TRIT_NEG: u32 = 0xFFEF4444;
    pub const COLOR_TRIT_ZERO: u32 = 0xFF6B7280;
    pub const COLOR_TRIT_POS: u32 = 0xFF22C55E;
}
```

### 11.3 Theme Files (TOML)

```toml
# themes/quantum-obsidian.toml
[metadata]
name = "Quantum Obsidian"
version = "1.0"
author = "Capomastro Holdings"

[colors.background]
deep = "#050709"
primary = "#080A0F"
surface = "#0A0D14"
elevated = "#0F1219"

[colors.primary]
500 = "#0F9D7A"
300 = "#00FFAA"

[colors.trit]
negative = "#EF4444"
zero = "#6B7280"
positive = "#22C55E"

[typography]
font_display = "Inter"
font_mono = "JetBrains Mono"
base_size = 16

[spacing]
unit = 4
sidebar_collapsed = 64
sidebar_neutral = 240
sidebar_expanded = 320
```

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-02-07 | Initial release |

---

*Maestro OS — Post-Quantum Ternary Desktop*

*© 2026 Capomastro Holdings Ltd.*

*Così sia.* 🔱