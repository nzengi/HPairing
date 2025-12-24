'use client'

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import { Server, Cpu, HardDrive, Zap, Clock } from 'lucide-react'
import { formatBytes, formatDuration } from '@/lib/utils'

export function SystemHealth() {
  // Mock system health data - gerçek uygulamada API'den gelecek
  const systemHealth = {
    uptime: 86400000 * 7, // 7 days in milliseconds
    memoryUsage: 0.68, // 68%
    storageUsage: 0.45, // 45%
    cpuUsage: 0.23, // 23%
    activeConnections: 42,
    responseTime: 45, // ms
    errorRate: 0.001, // 0.1%
    totalRequests: 125430,
    throughput: 1250 // req/sec
  }

  const getHealthColor = (value: number, thresholds: { warning: number, critical: number }) => {
    if (value >= thresholds.critical) return 'text-red-500'
    if (value >= thresholds.warning) return 'text-yellow-500'
    return 'text-green-500'
  }

  const getHealthBadge = (value: number, thresholds: { warning: number, critical: number }) => {
    if (value >= thresholds.critical) return 'destructive'
    if (value >= thresholds.warning) return 'secondary'
    return 'default'
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Server className="h-5 w-5" />
          System Health
        </CardTitle>
        <CardDescription>
          Real-time system performance and resource utilization
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-4 gap-4 md:gap-6">
            {/* Uptime */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Clock className="h-4 w-4 text-green-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Uptime</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold truncate">
                {formatDuration(systemHealth.uptime)}
              </div>
              <Badge variant="outline" className="w-fit text-xs">
                <div className="w-2 h-2 rounded-full bg-green-500 mr-1"></div>
                Healthy
              </Badge>
            </div>

            {/* Memory Usage */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Server className="h-4 w-4 text-blue-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Memory</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">
                {(systemHealth.memoryUsage * 100).toFixed(0)}%
              </div>
              <Progress value={systemHealth.memoryUsage * 100} className="h-2" />
              <div className="text-xs text-muted-foreground truncate">
                2.7GB / 4GB used
              </div>
            </div>

            {/* CPU Usage */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Cpu className="h-4 w-4 text-purple-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">CPU</span>
              </div>
              <div className={`text-xl sm:text-2xl font-bold ${getHealthColor(systemHealth.cpuUsage * 100, { warning: 70, critical: 90 })}`}>
                {(systemHealth.cpuUsage * 100).toFixed(0)}%
              </div>
              <Progress value={systemHealth.cpuUsage * 100} className="h-2" />
              <div className="text-xs text-muted-foreground truncate">
                4 cores active
              </div>
            </div>

            {/* Storage Usage */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <HardDrive className="h-4 w-4 text-orange-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Storage</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">
                {(systemHealth.storageUsage * 100).toFixed(0)}%
              </div>
              <Progress value={systemHealth.storageUsage * 100} className="h-2" />
              <div className="text-xs text-muted-foreground truncate">
                {formatBytes(450 * 1024 * 1024)} / 1TB
              </div>
            </div>

            {/* Active Connections */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Zap className="h-4 w-4 text-cyan-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Connections</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">
                {systemHealth.activeConnections}
              </div>
              <Badge variant="outline" className="w-fit text-xs">
                <div className="w-2 h-2 rounded-full bg-green-500 mr-1"></div>
                Stable
              </Badge>
            </div>

            {/* Response Time */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Clock className="h-4 w-4 text-indigo-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Response Time</span>
              </div>
              <div className={`text-xl sm:text-2xl font-bold ${getHealthColor(systemHealth.responseTime, { warning: 100, critical: 500 })}`}>
                {systemHealth.responseTime}ms
              </div>
              <Badge variant={getHealthBadge(systemHealth.responseTime, { warning: 100, critical: 500 }) as any} className="w-fit text-xs">
                Fast
              </Badge>
            </div>

            {/* Throughput */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Server className="h-4 w-4 text-emerald-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Throughput</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">
                {systemHealth.throughput}/s
              </div>
              <div className="text-xs text-muted-foreground truncate">
                Peak: 2.1k req/s
              </div>
            </div>

            {/* Error Rate */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Server className="h-4 w-4 text-red-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Error Rate</span>
              </div>
              <div className={`text-xl sm:text-2xl font-bold ${getHealthColor(systemHealth.errorRate * 100, { warning: 1, critical: 5 })}`}>
                {(systemHealth.errorRate * 100).toFixed(2)}%
              </div>
              <Badge variant={getHealthBadge(systemHealth.errorRate * 100, { warning: 1, critical: 5 }) as any} className="w-fit text-xs">
                Excellent
              </Badge>
            </div>
        </div>

        {/* Performance Summary */}
        <div className="mt-6 pt-6 border-t">
          <div className="grid grid-cols-1 sm:grid-cols-3 xl:grid-cols-3 gap-4 text-center">
            <div>
              <div className="text-2xl font-bold text-green-600">
                {systemHealth.totalRequests.toLocaleString()}
              </div>
              <div className="text-sm text-muted-foreground">Total Requests</div>
            </div>
            <div>
              <div className="text-2xl font-bold text-blue-600">99.99%</div>
              <div className="text-sm text-muted-foreground">Uptime</div>
            </div>
            <div>
              <div className="text-2xl font-bold text-purple-600">
                {systemHealth.activeConnections}
              </div>
              <div className="text-sm text-muted-foreground">Active Users</div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
