// HPair Enterprise Dashboard Types

export interface Group {
  id: string
  name: string
  participants: Participant[]
  createdAt: number
  lastActivity: number
  messageCount: number
  encryptionLevel: SecurityLevel
  complianceStatus: ComplianceStatus
  storageSize: number
}

export interface Participant {
  id: string
  name: string
  role: 'admin' | 'user' | 'auditor'
  joinedAt: number
  lastSeen: number
  publicKey?: string
}

export interface Message {
  id: string
  groupId: string
  senderId: string
  senderName: string
  content: string
  timestamp: number
  encryptionAlgorithm: string
  deliveryStatus: 'sent' | 'delivered' | 'failed'
}

export interface AuditLog {
  id: string
  timestamp: number
  action: string
  userId: string
  userName: string
  resourceType: 'group' | 'message' | 'user' | 'system'
  resourceId: string
  details: Record<string, any>
  ipAddress?: string
  userAgent?: string
}

export type SecurityLevel = 'quantum' | 'hybrid' | 'legacy'

export type ComplianceStatus = 'compliant' | 'warning' | 'critical' | 'pending'

export interface ComplianceMetrics {
  nistStatus: ComplianceStatus
  fipsValidated: boolean
  cnsaCompliant: boolean
  lastAudit: number
  nextAudit: number
  vulnerabilities: number
  encryptionStrength: number
}

export interface SystemMetrics {
  totalGroups: number
  totalUsers: number
  totalMessages: number
  activeConnections: number
  uptime: number
  memoryUsage: number
  storageUsage: number
  errorRate: number
}

export interface DashboardStats {
  groups: {
    total: number
    active: number
    compliance: {
      compliant: number
      warning: number
      critical: number
    }
  }
  messages: {
    total: number
    today: number
    encrypted: number
  }
  security: {
    encryptionStrength: number
    vulnerabilities: number
    lastSecurityScan: number
  }
  performance: {
    uptime: number
    responseTime: number
    throughput: number
  }
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
  timestamp: number
}

export interface PaginationParams {
  page: number
  limit: number
  sortBy?: string
  sortOrder?: 'asc' | 'desc'
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}
