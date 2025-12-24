'use client'

import { cn } from '@/lib/utils'
import { Badge } from '@/components/ui/badge'
import {
  Shield,
  LayoutDashboard,
  Users,
  ShieldCheck,
  FileText,
  Settings,
  X
} from 'lucide-react'

interface DashboardSidebarProps {
  activeView: string
  onViewChange: (view: string) => void
  isOpen: boolean
  onClose: () => void
}

const navigation = [
  {
    id: 'overview',
    name: 'Overview',
    icon: LayoutDashboard,
    description: 'Compliance dashboard'
  },
  {
    id: 'groups',
    name: 'Groups',
    icon: Users,
    description: 'Secure group management'
  },
  {
    id: 'security',
    name: 'Security',
    icon: ShieldCheck,
    description: 'Security metrics & compliance'
  },
  {
    id: 'audit',
    name: 'Audit Logs',
    icon: FileText,
    description: 'Security audit trail'
  },
  {
    id: 'settings',
    name: 'Settings',
    icon: Settings,
    description: 'System configuration'
  }
]

export function DashboardSidebar({
  activeView,
  onViewChange,
  isOpen,
  onClose
}: DashboardSidebarProps) {
  return (
    <>
      {/* Mobile backdrop */}
      {isOpen && (
        <div
          className="fixed inset-0 z-40 bg-background/80 backdrop-blur-sm lg:hidden"
          onClick={onClose}
        />
      )}

      {/* Sidebar */}
      <div className={cn(
        "fixed inset-y-0 left-0 z-50 w-64 bg-card border-r border-border transform transition-transform duration-300 ease-in-out lg:translate-x-0",
        isOpen ? "translate-x-0" : "-translate-x-full"
      )}>
        <div className="flex flex-col h-full max-h-screen">
          {/* Header */}
          <div className="flex items-center justify-between p-6 border-b border-border">
            <div className="flex items-center gap-2">
              <Shield className="h-6 w-6 text-primary" />
              <div>
                <h2 className="text-lg font-semibold">HPair</h2>
                <p className="text-xs text-muted-foreground">Enterprise</p>
              </div>
            </div>
            <button
              onClick={onClose}
              className="lg:hidden p-2 hover:bg-accent rounded-md"
            >
              <X className="h-5 w-5" />
            </button>
          </div>

          {/* Compliance Status */}
          <div className="p-4 border-b border-border">
            <div className="flex items-center gap-2">
              <Badge variant="compliance-safe">
                <Shield className="h-3 w-3 mr-1" />
                NIST Compliant
              </Badge>
            </div>
            <p className="text-xs text-muted-foreground mt-2">
              FIPS 140-3 • CNSA 2.0 • Post-Quantum Ready
            </p>
          </div>

          {/* Navigation */}
          <nav className="flex-1 p-4">
            <ul className="space-y-2">
              {navigation.map((item) => {
                const Icon = item.icon
                const isActive = activeView === item.id

                return (
                  <li key={item.id}>
                    <button
                      className={cn(
                        "w-full text-left p-3 rounded-md hover:bg-accent transition-colors",
                        isActive && "bg-secondary"
                      )}
                      onClick={() => {
                        onViewChange(item.id)
                        onClose()
                      }}
                    >
                      <div className="flex items-center gap-3">
                        <Icon className="h-5 w-5" />
                        <div className="flex flex-col items-start">
                          <span className="text-sm font-medium">{item.name}</span>
                          <span className="text-xs text-muted-foreground">
                            {item.description}
                          </span>
                        </div>
                      </div>
                    </button>
                  </li>
                )
              })}
            </ul>
          </nav>

          {/* Footer */}
          <div className="p-4 border-t border-border">
            <div className="text-xs text-muted-foreground space-y-1">
              <p>Version 1.0.0</p>
              <p>FIPS 140-3 Validated</p>
              <p>© 2025 HPair Enterprise</p>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}
