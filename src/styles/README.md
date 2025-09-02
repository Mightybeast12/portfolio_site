# CSS Organization Structure

This directory contains the reorganized CSS files for the portfolio site, structured for better maintainability and modularity.

## Directory Structure

```
src/styles/
├── index.css                    # Main entry point - imports all styles
├── base/
│   └── global.css              # Global styles, typography, base layout
├── components/
│   ├── buttons/
│   │   ├── button-4.css        # Neumorphic button style
│   │   ├── button-20.css       # Arrow button with animations
│   │   ├── button-54.css       # Shadow effect button
│   │   └── tooltip-button.css  # Complex tooltip button component
│   ├── cards/
│   │   ├── code-card-1.css     # Terminal-style code display card
│   │   └── code-card-2.css     # Modern card component
│   ├── images/
│   │   ├── carousel.css        # Image carousel component
│   │   └── zoom-image.css      # Zoom image component
│   └── navigation/
│       └── navbar.css          # Navigation bar styles
└── README.md                   # This documentation file
```

## Component Descriptions

### Base Styles
- **global.css**: Contains site-wide styles including color scheme, typography, base layout classes, and link styles.

### Button Components
- **button-4.css**: Neumorphic design button with smooth hover transitions
- **button-20.css**: Interactive arrow button with expanding border animation
- **button-54.css**: Retro-style button with layered shadow effects
- **tooltip-button.css**: Complex social media style button with layered hover effects and tooltip

### Card Components
- **code-card-1.css**: Terminal-style card with macOS-like traffic light controls for code display
- **code-card-2.css**: Clean modern card layout for content display

### Image Components
- **carousel.css**: Interactive image carousel with navigation controls
- **zoom-image.css**: Hover-to-zoom image containers with smooth transitions

### Navigation Components
- **navbar.css**: Responsive hamburger navigation with mobile-first design

## Usage

All styles are imported through the main `index.css` file. To use in your HTML:

```html
<link rel="stylesheet" href="src/styles/index.css">
```

## Adding New Components

1. Create a new CSS file in the appropriate `components/` subdirectory
2. Add the import statement to `index.css` in the correct section
3. Follow the existing naming conventions:
   - Use kebab-case for file names
   - Use descriptive class names with component prefixes
   - Include component-specific comments at the top of each file

## Migration from Old Structure

The old `static/css/` files have been reorganized as follows:

- `styles.css` → `base/global.css`
- `buttons.css` → `components/buttons/` (split into individual files)
- `cardstyles.css` → `components/cards/` (split into individual files)
- `imagestyle.css` → `components/images/` (split into individual files)
- `navigationstyles.css` → `components/navigation/navbar.css`
- `index.css` → `index.css` (updated to import new structure)

## Benefits of This Organization

1. **Modularity**: Each component is self-contained and easy to maintain
2. **Reusability**: Components can be easily reused across different pages
3. **Scalability**: Easy to add new components without cluttering
4. **Maintainability**: Changes to specific components don't affect others
5. **Performance**: Allows for potential code splitting and lazy loading
6. **Team Development**: Multiple developers can work on different components without conflicts
