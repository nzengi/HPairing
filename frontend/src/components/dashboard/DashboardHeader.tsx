'use client'

import { Menu, Bell, Shield, User } from 'lucide-react'
import { Badge } from '@/components/ui/badge'

interface DashboardHeaderProps {
  onMenuClick: () => void
}

export function DashboardHeader({ onMenuClick }: DashboardHeaderProps) {
  return (
    <header className="bg-card border-b border-border">
      <div className="w-full max-w-7xl mx-auto px-4 py-3 lg:px-6">
        <div className="flex items-center justify-between">
        {/* Left side - Menu button and branding */}
        <div className="flex items-center gap-4">
          <button
            onClick={onMenuClick}
            className="lg:hidden p-2 hover:bg-accent rounded-md"
          >
            <Menu className="h-5 w-5" />
          </button>

          <div className="flex items-center gap-2">
            <Shield className="h-6 w-6 text-primary" />
            <div className="hidden sm:block">
              <h1 className="text-lg font-semibold">HPair Enterprise</h1>
              <p className="text-xs text-muted-foreground">Compliance Dashboard</p>
            </div>
          </div>
        </div>

        {/* Right side - Status and basic user info */}
        <div className="flex items-center gap-4">
          {/* Compliance Status Badge */}
          <Badge variant="compliance-safe">
            <Shield className="h-3 w-3 mr-1" />
            NIST Compliant
          </Badge>

          {/* Notifications Badge */}
          <div className="relative">
            <Bell className="h-5 w-5 text-muted-foreground" />
            <span className="absolute -top-1 -right-1 h-4 w-4 bg-destructive rounded-full text-[10px] text-destructive-foreground flex items-center justify-center">
              3
            </span>
          </div>

          {/* Simple User Info */}
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-primary flex items-center justify-center text-primary-foreground text-sm font-medium">
              <User className="h-4 w-4" />
            </div>
            <div className="hidden md:block">
              <p className="text-sm font-medium">Security Admin</p>
              <p className="text-xs text-muted-foreground">admin@hpair-enterprise.com</p>
            </div>
          </div>
        </div>
        </div>
      </div>
    </header>
  )
}
