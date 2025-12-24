import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * Format bytes to human readable format
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes'

  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

/**
 * Format duration in milliseconds to human readable format
 */
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
  if (ms < 3600000) return `${(ms / 60000).toFixed(1)}m`
  return `${(ms / 3600000).toFixed(1)}h`
}

/**
 * Get compliance status color and label
 */
export function getComplianceStatus(status: string): {
  color: string
  label: string
  variant: 'safe' | 'warning' | 'critical' | 'pending'
} {
  switch (status.toLowerCase()) {
    case 'compliant':
    case 'safe':
      return { color: 'compliance-safe', label: 'NIST Compliant', variant: 'safe' }
    case 'warning':
    case 'review':
      return { color: 'compliance-warning', label: 'Review Needed', variant: 'warning' }
    case 'critical':
    case 'non-compliant':
      return { color: 'compliance-critical', label: 'Non-Compliant', variant: 'critical' }
    default:
      return { color: 'compliance-pending', label: 'In Progress', variant: 'pending' }
  }
}

/**
 * Get security level color and label
 */
export function getSecurityLevel(level: string): {
  color: string
  label: string
} {
  switch (level.toLowerCase()) {
    case 'quantum':
    case 'pqc':
      return { color: '#7c3aed', label: 'Post-Quantum' }
    case 'hybrid':
      return { color: '#0891b2', label: 'Hybrid Crypto' }
    case 'legacy':
      return { color: '#dc2626', label: 'Legacy/Vulnerable' }
    default:
      return { color: '#6b7280', label: 'Unknown' }
  }
}

/**
 * Validate group ID format
 */
export function isValidGroupId(id: string): boolean {
  return /^[a-f0-9]{16}$/i.test(id)
}

/**
 * Generate secure random group ID (for demo purposes)
 */
export function generateGroupId(): string {
  return Array.from({ length: 16 }, () =>
    Math.floor(Math.random() * 16).toString(16)
  ).join('')
}

/**
 * Format timestamp for display
 */
export function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  // Use a consistent format to avoid hydration mismatch
  const month = (date.getMonth() + 1).toString().padStart(2, '0')
  const day = date.getDate().toString().padStart(2, '0')
  const year = date.getFullYear()
  const hours = date.getHours().toString().padStart(2, '0')
  const minutes = date.getMinutes().toString().padStart(2, '0')

  return `${month}/${day}/${year} ${hours}:${minutes}`
}
