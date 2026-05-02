---
name: ringui
description: docs and demo for ringui, a react component lib
---

# Components
the concrete usage demo(.stories.tsx) are in skill reference.

| Component | Story File | Description |
|-----------|------------|-------------|
| alert-service | `alert-service.stories.tsx` | Service for managing a stack of alerts/messages that appear as notifications. |
| confirm | `confirm.stories.tsx` | A component that shows a confirmation dialog with confirm/cancel actions. |
| icon | `icon.stories.tsx` | Displays an icon using SVG glyphs. |
| select | `select.stories.tsx` | A dropdown select component with filtering, multiple selection, and customization options. |
| alert | `alert.stories.tsx` | Displays an alert/notification message with close button and different types (success, error, etc.). |
| content-layout | `content-layout.stories.tsx` | A component for simple content layout with optional sidebar. |
| input-size | `input-size.stories.tsx` | Input size styles (extra-short, short, medium, large). |
| simple-table | `simple-table.stories.tsx` | A simple stateless table without hover effect. |
| analytics | `analytics.stories.tsx` | Provides a façade to Google Analytics and other web analytics services through plugins. |
| contenteditable | `contenteditable.stories.tsx` | Provides a ContentEditable component for rich text editing. |
| input | `input.stories.tsx` | Text input fields of varying size. |
| island | `island.stories.tsx` | A card-like container with header and content sections. |
| link | `link.stories.tsx` | Displays a link. |
| slider | `slider.stories.tsx` | Displays a range slider input. |
| storage | `storage.stories.tsx` | Provides a façade to localStorage/sessionStorage/cookies. |
| auth-dialog-service | `auth-dialog-service.stories.tsx` | A wrapper for the AuthDialog component that allows showing the auth dialog without mounting the component first. Can be used outside React. |
| auth-dialog | `auth-dialog.stories.tsx` | A component that shows an authentication dialog. |
| auth | `auth.stories.tsx` | Authenticates a user in JetBrains Hub. |
| avatar-stack | `avatar-stack.stories.tsx` | Displays a stack of overlapping avatars. |
| avatar | `avatar.stories.tsx` | Displays an avatar image. Shows an empty square on loading error. |
| banner | `banner.stories.tsx` | Displays a prominent message banner with optional icon, title, and close button. |
| breadcrumbs | `breadcrumbs.stories.tsx` | Displays a breadcrumb navigation trail. |
| button-group | `button-group.stories.tsx` | Allows grouping several buttons together. |
| button-set | `button-set.stories.tsx` | Groups buttons with consistent margins between them. |
| button-toolbar | `button-toolbar.stories.tsx` | Displays a toolbar with several buttons. |
| button | `button.stories.tsx` | A clickable button component. |
| caret | `caret.stories.tsx` | Utility for manipulating caret/cursor position in text inputs and contenteditable elements. |
| checkbox | `checkbox.stories.tsx` | A checkbox input with label and optional help text. |
| clipboard | `clipboard.stories.tsx` | Utility for copying text to the clipboard. |
| code | `code.stories.tsx` | Displays a block of code with syntax highlighting. |
| collapse | `collapse.stories.tsx` | A component that hides content and expands to show more when clicked. |
| confirm-service | `confirm-service.stories.tsx` | A wrapper for the Confirm component that allows showing a confirmation dialog without mounting the component first. Can be used outside React. |
| data-list | `data-list.stories.tsx` | A component for rendering interactive hierarchical tables. |
| date-picker | `date-picker.stories.tsx` | Allows picking a date or date range. Uses date-fns under the hood. |
| dialog | `dialog.stories.tsx` | Presents content above an enclosing view in a modal overlay. |
| dom | `dom.stories.tsx` | A collection of DOM utilities. |
| dropdown-menu | `dropdown-menu.stories.tsx` | Displays a menu in a dropdown. |
| dropdown | `dropdown.stories.tsx` | A stateful popup with a clickable anchor. |
| editable-heading | `editable-heading.stories.tsx` | A component for rendering editable h1-h5 heading tags. |
| error-bubble | `error-bubble.stories.tsx` | Displays an error bubble near an input component when an error prop is provided. |
| error-message | `error-message.stories.tsx` | Displays an error message centered vertically and horizontally inside its parent container. |
| footer | `footer.stories.tsx` | A configurable page footer with left, center, and right sections. |
| form | `form.stories.tsx` | Helps create forms with various types of controls. |
| grid | `grid.stories.tsx` | A flexbox-based grid system for component layout. |
| group | `group.stories.tsx` | Places inner components with fixed spacing between them. |
| header | `header.stories.tsx` | A page header component with logo, navigation, and user profile. |
| heading | `heading.stories.tsx` | A component for rendering h1-h5 heading tags. |
| http | `http.stories.tsx` | Provides a way to perform authorized network requests. |
| i18n | `i18n.stories.tsx` | Internationalization service with translation and locale support. |
| list | `list.stories.tsx` | A selectable list component with support for custom items, avatars, and icons. |
| loader-inline | `loader-inline.stories.tsx` | A small animated loader displayed inline with text. |
| loader-screen | `loader-screen.stories.tsx` | A large animated loader with optional caption for page/major action loading. |
| loader | `loader.stories.tsx` | A generic animated loader with optional message. |
| markdown | `markdown.stories.tsx` | Renders markdown content with optional syntax highlighting. |
| message | `message.stories.tsx` | Displays a tooltip-like message anchored to an element. |
| old-browsers-message | `old-browsers-message.stories.tsx` | Displays a full-screen "Browser is unsupported" message for old browsers. |
| pager | `pager.stories.tsx` | Displays pagination controls for navigating between pages. |
| panel | `panel.stories.tsx` | Displays a button panel for actions. |
| popup-menu | `popup-menu.stories.tsx` | Displays a popup menu. |
| popup | `popup.stories.tsx` | A positioned popup container that can anchor to elements. |
| progress-bar | `progress-bar.stories.tsx` | Displays a progress bar indicator. |
| query-assist | `query-assist.stories.tsx` | A query input with search assistance/autocomplete suggestions. |
| radio | `radio.stories.tsx` | Displays a radio button group for single selection. |
| scrollable-section | `scrollable-section.stories.tsx` | A scrollable container section. |
| tab-trap | `tab-trap.stories.tsx` | Restricts tab navigation to stay within a designated area. |
| table | `table.stories.tsx` | A feature-rich table component with sorting, selection, and pagination. |
| tabs | `tabs.stories.tsx` | Displays a tab set for navigating between content sections. |
| tag-input | `tag-input.stories.tsx` | A tags input field for entering multiple values. |
| tag | `tag.stories.tsx` | Displays a tag/label with optional remove button. |
| tags-list | `tags-list.stories.tsx` | Displays a list of tags. |
| text | `text.stories.tsx` | A component for rendering styled text content. |
| theme-provider | `theme.stories.tsx` | Provides theme context (light/dark) for child components. |
| toggle | `toggle.stories.tsx` | Displays an animated on/off toggle switch. |
| tooltip | `tooltip.stories.tsx` | Displays a tooltip on hover. |
| upload | `upload.stories.tsx` | A file-upload dropzone component. |
| user-agreement | `user-agreement.stories.tsx` | Displays a user agreement/EULA dialog. |
| user-card | `user-card.stories.tsx` | A component that displays user details. |
| theme-palette | `variables.stories.tsx` | Shows the Ring UI color palette based on CSS custom properties. |

# Guideline

- ALWAYS prefer Ring UI components over custom implementations.
- ALWAYS prefer Ring UI builtin palete color over hardcode color
- ALWAYS prefer Ring UI  Group, Island ... as container, INSTEAD OF  div,span,...
