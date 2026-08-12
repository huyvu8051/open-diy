https://chatgpt.com/share/6a7cc42a-bb8c-83ec-a43a-5de63001bc0b


Được. Nếu mục tiêu của bạn là dùng **Stitch để generate một bộ UI/template đủ rộng để sau đó ráp thành một ứng dụng hoàn chỉnh**, thì đừng chỉ generate các page chính như Home / Dashboard / Settings. Nên chuẩn bị cả **happy path, empty state, error state, auth, onboarding, CRUD, admin, billing, notification, search, permission, system pages**.

Dưới đây là một **master checklist** mình sẽ dùng để yêu cầu Stitch generate toàn bộ UI.

## 1. Public / Marketing

* [ ] Landing / Home
* [ ] Features
* [ ] Feature detail
* [ ] Product overview
* [ ] Pricing
* [ ] Pricing comparison
* [ ] About
* [ ] Team
* [ ] Careers
* [ ] Job detail
* [ ] Blog listing
* [ ] Blog detail
* [ ] Documentation landing
* [ ] Documentation category
* [ ] Documentation article
* [ ] FAQ
* [ ] Contact
* [ ] Contact success
* [ ] Partners
* [ ] Integrations
* [ ] Integration detail
* [ ] Changelog
* [ ] Roadmap
* [ ] Testimonials / Customers
* [ ] Case studies
* [ ] Case study detail
* [ ] Press / Media
* [ ] Legal
* [ ] Privacy Policy
* [ ] Terms of Service
* [ ] Cookie Policy

---

# 2. Authentication

Đây là nhóm **bắt buộc** nếu muốn template có thể dùng cho app thật.

* [ ] Login
* [ ] Login with email/password
* [ ] Login with Google
* [ ] Login with Apple
* [ ] Login with GitHub
* [ ] Sign up
* [ ] Sign up confirmation
* [ ] Email verification
* [ ] Verification success
* [ ] Verification expired
* [ ] Forgot password
* [ ] Reset password
* [ ] Reset password success
* [ ] Magic link
* [ ] Magic link sent
* [ ] 2FA setup
* [ ] 2FA verification
* [ ] 2FA backup codes
* [ ] 2FA recovery
* [ ] Passkey setup
* [ ] Session expired
* [ ] Account locked
* [ ] Suspicious login
* [ ] Logout confirmation
* [ ] Invite acceptance
* [ ] SSO login
* [ ] SSO error
* [ ] Organization selection

---

# 3. Onboarding

* [ ] Welcome
* [ ] Choose role
* [ ] Choose use case
* [ ] Choose interests
* [ ] Profile setup
* [ ] Company setup
* [ ] Workspace creation
* [ ] Workspace configuration
* [ ] Import data
* [ ] Connect integrations
* [ ] Invite teammates
* [ ] Configure preferences
* [ ] Tutorial
* [ ] Product walkthrough
* [ ] Onboarding checklist
* [ ] Onboarding completion
* [ ] Skip onboarding confirmation

---

# 4. Main Application Shell

Nên yêu cầu Stitch tạo **nhiều biến thể layout**, không chỉ một dashboard.

* [ ] Desktop sidebar layout
* [ ] Collapsed sidebar
* [ ] Mobile navigation
* [ ] Top navigation
* [ ] Bottom navigation
* [ ] Breadcrumb navigation
* [ ] Tabs
* [ ] Nested tabs
* [ ] Command palette
* [ ] Global search
* [ ] User menu
* [ ] Workspace switcher
* [ ] Organization switcher
* [ ] Notification center
* [ ] Help center
* [ ] Global action button
* [ ] Contextual action menu

# reached here
---

# 5. Dashboard

Dashboard thường là page cần nhiều state nhất.

* [ ] Main dashboard
* [ ] Analytics dashboard
* [ ] Admin dashboard
* [ ] User dashboard
* [ ] Team dashboard
* [ ] Executive dashboard
* [ ] KPI dashboard
* [ ] Revenue dashboard
* [ ] Activity dashboard
* [ ] Dashboard customization
* [ ] Widget selection
* [ ] Widget configuration
* [ ] Date range selector
* [ ] Filters
* [ ] Saved filters
* [ ] Export dashboard
* [ ] Share dashboard

### Dashboard states

* [ ] Normal
* [ ] Loading
* [ ] Skeleton loading
* [ ] Empty
* [ ] No data
* [ ] Partial data
* [ ] Error
* [ ] Permission denied
* [ ] Offline

---

# 6. CRUD Pages

Đây là nhóm cực kỳ quan trọng để app có thể phát triển thành sản phẩm thật.

Với **mỗi resource/entity**, nên có:

### List

* [ ] Resource list
* [ ] Table view
* [ ] Grid view
* [ ] Card view
* [ ] Kanban view
* [ ] List filters
* [ ] Search
* [ ] Sort
* [ ] Pagination
* [ ] Infinite scroll
* [ ] Column customization
* [ ] Saved views
* [ ] Bulk selection
* [ ] Bulk actions
* [ ] Export
* [ ] Import

### Create

* [ ] Create page
* [ ] Create modal
* [ ] Multi-step create
* [ ] Form validation
* [ ] Draft state
* [ ] Save & continue
* [ ] Save & close

### Detail

* [ ] Detail page
* [ ] Overview
* [ ] Activity
* [ ] History
* [ ] Related records
* [ ] Attachments
* [ ] Comments
* [ ] Metadata

### Edit

* [ ] Edit page
* [ ] Inline edit
* [ ] Edit modal
* [ ] Unsaved changes
* [ ] Save confirmation

### Delete

* [ ] Delete confirmation
* [ ] Soft delete
* [ ] Trash
* [ ] Restore
* [ ] Permanent delete confirmation

---

# 7. Search

* [ ] Global search
* [ ] Search results
* [ ] Search suggestions
* [ ] Recent searches
* [ ] Search history
* [ ] Advanced search
* [ ] Search filters
* [ ] Search by category
* [ ] Search by user
* [ ] Search by date
* [ ] Search no results
* [ ] Search error
* [ ] Search loading

---

# 8. User Profile

* [ ] Profile overview
* [ ] Edit profile
* [ ] Avatar upload
* [ ] Personal information
* [ ] Contact information
* [ ] Password
* [ ] Security
* [ ] Sessions
* [ ] Login history
* [ ] Connected accounts
* [ ] Connected apps
* [ ] Preferences
* [ ] Language
* [ ] Timezone
* [ ] Appearance
* [ ] Accessibility

---

# 9. Settings

Nên generate Settings theo **category**, thay vì một page duy nhất.

### General

* [ ] General settings
* [ ] Application settings
* [ ] Workspace settings
* [ ] Organization settings

### Account

* [ ] Account
* [ ] Profile
* [ ] Password
* [Security](#)

### Notifications

* [ ] Notification preferences
* [ ] Email notifications
* [ ] Push notifications
* [ ] SMS notifications
* [ ] In-app notifications
* [ ] Digest settings

### Appearance

* [ ] Light mode
* [ ] Dark mode
* [ ] System theme
* [ ] Density
* [ ] Layout settings

### Localization

* [ ] Language
* [ ] Region
* [ ] Currency
* [ ] Timezone
* [ ] Date format

---

# 10. Team / Organization

Nếu app có multi-user thì nhóm này rất đáng làm.

* [ ] Team overview
* [ ] Members
* [ ] Member detail
* [ ] Invite member
* [ ] Invite pending
* [ ] Resend invitation
* [ ] Remove member
* [ ] Change role
* [ ] Teams
* [ ] Team detail
* [ ] Groups
* [ ] Departments
* [ ] Organization profile
* [ ] Organization settings
* [ ] Workspace management
* [ ] Workspace switcher

---

# 11. Roles & Permissions

* [ ] Roles list
* [ ] Role detail
* [ ] Create role
* [ ] Edit role
* [ ] Permission matrix
* [ ] User permissions
* [ ] Team permissions
* [ ] Resource permissions
* [ ] Admin permissions
* [ ] Access denied
* [ ] Request access
* [ ] Access request pending

---

# 12. Notifications

* [ ] Notification center
* [ ] Notification list
* [ ] Notification detail
* [ ] Unread state
* [ ] Read state
* [ ] Mark all read
* [ ] Notification preferences
* [ ] Notification settings
* [ ] Activity feed
* [ ] Mentions
* [ ] System alerts
* [ ] Success notification
* [ ] Warning notification
* [ ] Error notification

---

# 13. Billing / Subscription

Nếu là SaaS thì gần như bắt buộc.

* [ ] Pricing
* [ ] Plan selection
* [ ] Checkout
* [ ] Payment method
* [ ] Billing overview
* [ ] Current plan
* [ ] Upgrade
* [ ] Downgrade
* [ ] Cancel subscription
* [ ] Cancellation confirmation
* [ ] Subscription paused
* [ ] Subscription expired
* [ ] Invoice list
* [ ] Invoice detail
* [ ] Invoice download
* [ ] Billing history
* [ ] Payment history
* [ ] Failed payment
* [ ] Update payment method
* [ ] Tax information
* [ ] Billing address
* [ ] Usage
* [ ] Usage limits
* [ ] Usage exceeded
* [ ] Free trial
* [ ] Trial ending
* [ ] Coupon / promo code

---

# 14. Files / Documents

Nếu app có file:

* [ ] File browser
* [ ] Folder browser
* [ ] File detail
* [ ] File preview
* [ ] Upload
* [ ] Upload progress
* [ ] Upload failed
* [ ] Drag & drop upload
* [ ] Create folder
* [ ] Rename
* [ ] Move
* [ ] Copy
* [ ] Download
* [ ] Share
* [ ] Permission management
* [ ] Version history
* [ ] Trash
* [ ] Storage usage

---

# 15. Collaboration

* [ ] Comments
* [ ] Comment thread
* [ ] Reply
* [ ] Mentions
* [ ] Reactions
* [ ] Activity feed
* [ ] Change history
* [ ] Version history
* [ ] Presence indicator
* [ ] Shared workspace
* [ ] Share modal
* [ ] Public link
* [ ] Link permissions
* [ ] Collaboration invitation

---

# 16. Messaging / Chat

Nếu application có communication:

* [ ] Inbox
* [ ] Conversation list
* [ ] Conversation detail
* [ ] New conversation
* [ ] Group conversation
* [ ] User profile in chat
* [ ] Attachments
* [ ] Emoji/reactions
* [ ] Reply
* [ ] Forward
* [ ] Search messages
* [ ] Pinned messages
* [ ] Archived conversation
* [ ] Unread conversation
* [ ] Empty inbox

---

# 17. Calendar / Scheduling

* [ ] Calendar month
* [ ] Calendar week
* [ ] Calendar day
* [ ] Agenda
* [ ] Event detail
* [ ] Create event
* [ ] Edit event
* [ ] Delete event
* [ ] Recurring event
* [ ] Invite attendees
* [ ] Availability
* [ ] Scheduling
* [ ] Reminder
* [ ] Meeting detail
* [ ] Calendar settings

---

# 18. Analytics / Reports

* [ ] Analytics overview
* [ ] Detailed analytics
* [ ] Report list
* [ ] Report detail
* [ ] Create report
* [ ] Report builder
* [ ] Custom metrics
* [ ] Filters
* [ ] Date ranges
* [ ] Comparison periods
* [ ] Charts
* [ ] Tables
* [ ] Export CSV
* [ ] Export PDF
* [ ] Schedule report
* [ ] Share report

---

# 19. Admin Panel

Đây là nhóm nhiều người hay quên.

* [ ] Admin dashboard
* [ ] User management
* [ ] User detail
* [ ] User impersonation
* [ ] Organization management
* [ ] Content management
* [ ] Role management
* [ ] Permission management
* [ ] Feature flags
* [ ] System configuration
* [ ] Audit logs
* [ ] Login logs
* [ ] API usage
* [ ] Webhooks
* [ ] Integrations
* [ ] System health
* [ ] Error logs
* [ ] Moderation queue
* [ ] Reports
* [ ] Support tickets

---

# 20. Integrations

* [ ] Integration marketplace
* [ ] Integration list
* [ ] Integration detail
* [ ] Connect integration
* [ ] OAuth authorization
* [ ] Connected state
* [ ] Disconnected state
* [ ] Integration settings
* [ ] API key
* [ ] Webhook configuration
* [ ] Sync status
* [ ] Sync error
* [ ] Integration logs

---

# 21. API / Developer

Nếu app có developer ecosystem:

* [ ] Developer dashboard
* [ ] API keys
* [ ] Create API key
* [ ] API key reveal
* [ ] API key revoke
* [ ] OAuth apps
* [ ] Create OAuth app
* [ ] Webhooks
* [ ] Webhook detail
* [ ] Webhook logs
* [ ] API usage
* [ ] API limits
* [ ] Documentation
* [ ] SDK documentation
* [ ] API reference

---

# 22. E-commerce nếu có

Nếu muốn bộ template có thể cover cả commerce:

* [ ] Product listing
* [ ] Product detail
* [ ] Category
* [ ] Search
* [ ] Cart
* [ ] Checkout
* [ ] Address
* [ ] Payment
* [ ] Order confirmation
* [ ] Order history
* [ ] Order detail
* [ ] Wishlist
* [ ] Reviews
* [ ] Coupons
* [ ] Returns
* [ ] Refund
* [ ] Shipping tracking
* [ ] Customer account
* [ ] Seller dashboard
* [ ] Inventory
* [ ] Product management
* [ ] Order management

---

# 23. Support

* [ ] Help center
* [ ] Knowledge base
* [ ] FAQ
* [ ] Support inbox
* [ ] Create ticket
* [ ] Ticket detail
* [ ] Ticket status
* [ ] Ticket history
* [ ] Live chat
* [ ] Contact support
* [ ] Report a problem
* [ ] Feature request
* [ ] Feedback

---

# 24. System Pages

**Rất quan trọng để template thực sự "production-ready".**

* [ ] 404
* [ ] 403 Forbidden
* [ ] 401 Unauthorized
* [ ] 500 Server Error
* [ ] 502 Bad Gateway
* [ ] 503 Service Unavailable
* [ ] Maintenance
* [ ] Coming Soon
* [ ] Offline
* [ ] Network Error
* [ ] Session Expired
* [ ] Browser Unsupported
* [ ] Update Required

---

# 25. Empty States

Mỗi feature nên có empty state riêng.

* [ ] No data
* [ ] No results
* [ ] No projects
* [ ] No team members
* [ ] No notifications
* [ ] No messages
* [ ] No files
* [ ] No integrations
* [ ] No transactions
* [ ] No activity
* [ ] No search results
* [ ] First-time empty state
* [ ] Permission-based empty state

---

# 26. Loading States

Đừng chỉ generate page static.

* [ ] Full-page loading
* [ ] Skeleton page
* [ ] Skeleton table
* [ ] Skeleton cards
* [ ] Skeleton dashboard
* [ ] Button loading
* [ ] Form submission loading
* [ ] Upload loading
* [ ] Infinite-scroll loading
* [ ] Lazy-load state
* [ ] Refreshing state

---

# 27. Error States

* [ ] Inline validation
* [ ] Form error
* [ ] API error
* [ ] Network error
* [ ] Permission error
* [ ] Upload error
* [ ] Payment error
* [ ] Authentication error
* [ ] Integration error
* [ ] Retry state
* [ ] Partial failure
* [ ] Recoverable error
* [ ] Fatal error

---

# 28. Modal / Drawer Templates

Nên generate riêng vì sau này dùng rất nhiều.

* [ ] Confirmation modal
* [ ] Delete modal
* [ ] Create modal
* [ ] Edit modal
* [ ] Form modal
* [ ] Warning modal
* [ ] Success modal
* [ ] Error modal
* [ ] Share modal
* [ ] Invite modal
* [ ] Upgrade modal
* [ ] Payment modal
* [ ] Command palette
* [ ] Filter drawer
* [ ] Detail drawer
* [ ] Mobile bottom sheet
* [ ] Full-screen modal

---

# 29. Forms

* [ ] Simple form
* [ ] Long form
* [ ] Multi-step form
* [ ] Conditional form
* [ ] Searchable select
* [ ] Multi-select
* [ ] Date picker
* [ ] Date range
* [ ] Time picker
* [ ] File upload
* [ ] Rich text editor
* [ ] Markdown editor
* [ ] Tags input
* [ ] Slider
* [ ] Toggle
* [ ] Radio
* [ ] Checkbox
* [ ] OTP input
* [ ] Password field
* [ ] Form autosave
* [ ] Form draft
* [ ] Unsaved changes
* [ ] Validation success
* [ ] Validation errors

---

# 30. Data-heavy UI

Nếu bạn muốn sau này làm SaaS/admin/ERP thì rất đáng generate:

* [ ] Data table
* [ ] Sortable table
* [ ] Filterable table
* [ ] Resizable columns
* [ ] Reorderable columns
* [ ] Column visibility
* [ ] Bulk actions
* [ ] Pagination
* [ ] Cursor pagination
* [ ] Infinite scroll
* [ ] Sticky headers
* [ ] Expandable rows
* [ ] Nested rows
* [ ] Comparison table
* [ ] Kanban
* [ ] Timeline
* [ ] Activity feed
* [ ] Tree view

---

# 31. Responsive

Mỗi page quan trọng nên có:

* [ ] Desktop
* [ ] Tablet
* [ ] Mobile
* [ ] Mobile navigation
* [ ] Mobile bottom sheet
* [ ] Mobile filters
* [ ] Mobile table transformation
* [ ] Mobile form
* [ ] Mobile dashboard

**Đặc biệt:** đừng chỉ yêu cầu Stitch "make responsive". Hãy yêu cầu nó tạo **mobile-specific states/layout**, vì table, sidebar, filter, modal… thường cần thay đổi cấu trúc chứ không chỉ co width.

---

# 32. Accessibility

Template nên bao phủ:

* [ ] Keyboard navigation
* [ ] Focus state
* [ ] Focus trap modal
* [ ] Screen-reader labels
* [ ] Error announcements
* [ ] Accessible forms
* [ ] Accessible tables
* [ ] High contrast
* [ ] Reduced motion
* [ ] Large text
* [ ] Touch-friendly controls

---

# 33. Các state UI nên yêu cầu cho **mọi component**

Đây là phần mình đặc biệt khuyên bạn đưa vào prompt Stitch:

**Mỗi component/page không chỉ có "default state".**

Generate:

* [ ] Default
* [ ] Hover
* [ ] Focus
* [ ] Active
* [ ] Selected
* [ ] Disabled
* [ ] Loading
* [ ] Success
* [ ] Warning
* [ ] Error
* [ ] Empty
* [ ] Read-only
* [ ] Permission denied
* [ ] Mobile
* [ ] Dark mode

---

# 34. Nếu muốn "website → application template" thật sự đầy đủ

Mình sẽ chia bộ Stitch thành khoảng **12 nhóm project** thay vì cố nhồi tất cả vào một generation:

1. **Marketing Website**
2. **Authentication**
3. **Onboarding**
4. **Core Application**
5. **CRUD / Data Management**
6. **User / Team / Organization**
7. **Settings**
8. **Billing**
9. **Notifications / Communication**
10. **Analytics / Reports**
11. **Admin**
12. **System / Error / Empty / Loading states**

Và mỗi nhóm nên có cả **desktop + mobile + dark mode + loading + empty + error + permission states**.

### Một cấu trúc page tree rất thực dụng

```text
/
├── marketing
│   ├── home
│   ├── features
│   ├── pricing
│   ├── about
│   ├── blog
│   ├── docs
│   └── contact
│
├── auth
│   ├── login
│   ├── signup
│   ├── verify-email
│   ├── forgot-password
│   ├── reset-password
│   ├── 2fa
│   └── invite
│
├── onboarding
│   ├── welcome
│   ├── profile
│   ├── workspace
│   ├── integrations
│   ├── invite-team
│   └── complete
│
├── app
│   ├── dashboard
│   ├── search
│   ├── notifications
│   ├── activity
│   ├── resources
│   │   ├── list
│   │   ├── create
│   │   ├── detail
│   │   └── edit
│   ├── analytics
│   ├── reports
│   └── calendar
│
├── workspace
│   ├── members
│   ├── teams
│   ├── roles
│   ├── permissions
│   └── integrations
│
├── settings
│   ├── general
│   ├── profile
│   ├── security
│   ├── notifications
│   ├── appearance
│   └── billing
│
├── billing
│   ├── overview
│   ├── plans
│   ├── checkout
│   ├── payment-method
│   ├── invoices
│   └── usage
│
├── admin
│   ├── dashboard
│   ├── users
│   ├── organizations
│   ├── permissions
│   ├── audit-logs
│   ├── integrations
│   └── system
│
└── system
    ├── 401
    ├── 403
    ├── 404
    ├── 500
    ├── maintenance
    ├── offline
    └── session-expired
```

## Quan trọng nhất khi prompt Stitch

Thay vì nói:

> "Create a SaaS dashboard."

hãy yêu cầu theo kiểu:

> **Generate a complete production-ready UI system for a SaaS application. Cover every realistic page, user flow, CRUD operation, navigation state, permission state, empty state, loading state, error state, confirmation state, onboarding state, billing state, admin state, and system state. Generate desktop, tablet and mobile layouts, including light and dark themes. Do not only design the happy path. Include all screens required to turn the design into a complete application template.**

Sau đó feed cho nó **12 nhóm ở trên từng nhóm một**. Cách này sẽ cho bạn một "UI library/page library" có tính tái sử dụng cao hơn nhiều so với yêu cầu Stitch generate toàn bộ website trong một shot.

Nếu mục tiêu của bạn là **dùng Stitch như một máy generate UI để sau đó code thành app**, thì mình còn khuyên thêm một tầng nữa: tạo **component inventory + page inventory + state inventory + user-flow inventory**. Khi 4 inventory này khớp nhau, bạn gần như có một **design system + application blueprint**, chứ không chỉ có một đống screenshot đẹp.

