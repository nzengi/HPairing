# HPair Enterprise Dashboard

Post-quantum secure multi-party communication platform with enterprise compliance dashboard.

## 🚀 Features

- **NIST FIPS 203-205 Compliant**: ML-KEM and ML-DSA post-quantum cryptography
- **Enterprise Compliance**: CNSA 2.0, HIPAA, GDPR ready
- **Real-time Monitoring**: Prometheus metrics and health checks
- **Security Audit Trail**: Complete audit logging
- **Group Management**: Secure multi-party communication
- **Role-based Access**: Admin, user, auditor roles

## 🛠️ Tech Stack

- **Framework**: Next.js 14 with App Router
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **UI Components**: Radix UI + shadcn/ui
- **Icons**: Lucide React
- **Charts**: Recharts (future)
- **State**: React hooks
- **Backend**: REST API integration

## 📦 Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

## 🎨 Design System

### Color Palette
- **Primary**: Navy Blue (#1a365d) - Trust, Security, Professional
- **Compliance Safe**: Green (#10b981) - NIST Compliant
- **Warning**: Amber (#f59e0b) - Review Needed
- **Critical**: Red (#ef4444) - Non-Compliant

### Typography
- **Primary Font**: Inter (system font stack)
- **Security Level**: High contrast for accessibility
- **Compliance Status**: Clear visual hierarchy

## 🏗️ Architecture

```
frontend/
├── src/
│   ├── app/                 # Next.js App Router
│   │   ├── globals.css      # Global styles
│   │   ├── layout.tsx       # Root layout
│   │   └── page.tsx         # Main dashboard
│   ├── components/
│   │   ├── ui/              # Reusable UI components
│   │   └── dashboard/       # Dashboard-specific components
│   ├── lib/
│   │   ├── utils.ts         # Utility functions
│   │   └── api.ts           # API client (future)
│   └── types/               # TypeScript type definitions
├── tailwind.config.ts       # Tailwind configuration
└── package.json
```

## 🔒 Security Features

### Frontend Security
- **CSP Headers**: Content Security Policy
- **Input Validation**: All user inputs validated
- **XSS Protection**: React's built-in XSS protection
- **Secure Defaults**: HTTPS-only, secure cookies

### Compliance Features
- **NIST Status Indicators**: Real-time compliance status
- **Audit Trail**: Complete activity logging
- **Access Control**: Role-based permissions
- **Security Metrics**: Vulnerability tracking

## 📊 Dashboard Sections

### 1. Overview
- Compliance status summary
- Security metrics overview
- System health indicators

### 2. Groups
- Secure group management
- Participant administration
- Message encryption status

### 3. Security
- Detailed security metrics
- Vulnerability assessment
- Compliance audit results

### 4. Audit Logs
- Security event timeline
- User activity tracking
- System audit trail

### 5. Settings
- Compliance configuration
- Security policies
- System preferences

## 🔌 API Integration

The dashboard integrates with the HPair backend API:

```typescript
// Example API calls
const complianceStatus = await api.getComplianceStatus()
const groups = await api.listGroups()
const auditLogs = await api.getAuditLogs({ page: 1, limit: 50 })
const metrics = await api.getMetrics()
```

## 🚀 Deployment

### Development
```bash
npm run dev
# Open http://localhost:3000
```

### Production
```bash
npm run build
npm start
# Or deploy to Vercel, Netlify, etc.
```

### Docker (Future)
```bash
docker build -t hpair-dashboard .
docker run -p 3000:3000 hpair-dashboard
```

## 📈 Performance

- **Bundle Size**: Optimized with Next.js
- **Loading**: Fast initial page loads
- **Caching**: Intelligent caching strategies
- **Monitoring**: Built-in performance metrics

## 🧪 Testing

```bash
# Run tests
npm test

# Run linting
npm run lint

# Type checking
npm run type-check
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔗 Links

- [HPair Backend API](https://github.com/nzengi/HPair)
- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [CNSA 2.0 Guidelines](https://www.cisa.gov/news-events/directives/cnsa-20-migration-guidance)

---

**Built for Enterprise Security, Designed for Compliance** 🛡️✨
