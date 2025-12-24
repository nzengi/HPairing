'use client'

import { useState } from 'react'
import { DashboardHeader } from '@/components/dashboard/DashboardHeader'
import { DashboardSidebar } from '@/components/dashboard/DashboardSidebar'
import { ComplianceOverview } from '@/components/dashboard/ComplianceOverview'
import { SecurityMetrics } from '@/components/dashboard/SecurityMetrics'
import { GroupManagement } from '@/components/dashboard/GroupManagement'
import { AuditLogs } from '@/components/dashboard/AuditLogs'
import { SystemHealth } from '@/components/dashboard/SystemHealth'

type ActiveView = 'overview' | 'groups' | 'security' | 'audit' | 'settings'

export default function DashboardPage() {
  const [activeView, setActiveView] = useState<ActiveView>('overview')
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const renderActiveView = () => {
    switch (activeView) {
      case 'overview':
        return (
          <div className="space-y-6">
            <ComplianceOverview />
            <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
              <SecurityMetrics />
              <SystemHealth />
            </div>
          </div>
        )
      case 'groups':
        return <GroupManagement />
      case 'security':
        return (
          <div className="space-y-6">
            <SecurityMetrics detailed />
            <ComplianceOverview detailed />
          </div>
        )
      case 'audit':
        return <AuditLogs />
      case 'settings':
        return (
          <div className="p-6">
            <div className="bg-card rounded-lg border p-6">
              <h2 className="text-2xl font-bold mb-4">System Settings</h2>
              <p className="text-muted-foreground">
                Enterprise compliance settings and system configuration.
              </p>
              <div className="mt-4 p-4 bg-muted rounded">
                Settings panel will be implemented in Phase 2.
              </div>
            </div>
          </div>
        )
      default:
        return <ComplianceOverview />
    }
  }

  return (
    <div className="min-h-screen bg-background overflow-x-hidden">
      <DashboardSidebar
        activeView={activeView}
        onViewChange={setActiveView}
        isOpen={sidebarOpen}
        onClose={() => setSidebarOpen(false)}
      />

      <div className="lg:pl-64">
        <DashboardHeader
          onMenuClick={() => setSidebarOpen(true)}
        />

        <main className="py-4 md:py-6">
          <div className="w-full max-w-7xl mx-auto px-3 sm:px-4 md:px-6 lg:px-8 xl:px-8 2xl:px-8">
            {renderActiveView()}
          </div>
        </main>
      </div>
    </div>
  )
}
