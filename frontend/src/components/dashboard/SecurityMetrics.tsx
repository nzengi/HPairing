'use client'

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import { Shield, Lock, Zap, Eye, AlertTriangle } from 'lucide-react'
import { getSecurityLevel } from '@/lib/utils'

interface SecurityMetricsProps {
  detailed?: boolean
}

export function SecurityMetrics({ detailed = false }: SecurityMetricsProps) {
  // Mock security metrics - gerçek uygulamada API'den gelecek
  const securityMetrics = {
    encryptionStrength: 256,
    activeThreats: 0,
    securityScore: 98,
    lastScan: Date.now() - 3600000, // 1 saat önce
    vulnerabilities: {
      critical: 0,
      high: 0,
      medium: 1,
      low: 2
    },
    encryptionAlgorithms: [
      { name: 'ML-KEM-1024', status: 'active', strength: 'quantum' },
      { name: 'ML-DSA-87', status: 'active', strength: 'quantum' },
      { name: 'AES-256-GCM', status: 'active', strength: 'hybrid' },
      { name: 'SHA3-256', status: 'active', strength: 'hybrid' }
    ],
    securityEvents: [
      { type: 'scan', message: 'Daily security scan completed', time: '1h ago' },
      { type: 'update', message: 'Key rotation completed', time: '3h ago' },
      { type: 'audit', message: 'Compliance audit passed', time: '1d ago' }
    ]
  }

  return (
    <div className="space-y-6">
      {/* Security Score Overview */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Shield className="h-5 w-5" />
            Security Metrics
          </CardTitle>
          <CardDescription>
            Real-time security posture and threat intelligence
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-4 gap-4 md:gap-6">
            {/* Security Score */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Shield className="h-4 w-4 text-green-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Security Score</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">{securityMetrics.securityScore}%</div>
              <Progress value={securityMetrics.securityScore} className="h-2" />
            </div>

            {/* Encryption Strength */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Lock className="h-4 w-4 text-blue-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Encryption</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">{securityMetrics.encryptionStrength}-bit</div>
              <Badge style={{ backgroundColor: getSecurityLevel('quantum').color }} className="text-xs">
                Post-Quantum
              </Badge>
            </div>

            {/* Active Threats */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <AlertTriangle className="h-4 w-4 text-red-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Active Threats</span>
              </div>
              <div className="text-xl sm:text-2xl font-bold">{securityMetrics.activeThreats}</div>
              <Badge variant="outline" className="w-fit text-xs">
                <Eye className="h-3 w-3 mr-1" />
                Monitored
              </Badge>
            </div>

            {/* Last Scan */}
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Zap className="h-4 w-4 text-yellow-500 flex-shrink-0" />
                <span className="text-sm font-medium truncate">Last Scan</span>
              </div>
              <div className="text-lg sm:text-2xl font-bold">1h ago</div>
              <Badge variant="outline" className="w-fit text-xs">
                <Shield className="h-3 w-3 mr-1" />
                Clean
              </Badge>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Detailed Security Information */}
      {detailed && (
        <>
          {/* Vulnerabilities */}
          <Card>
            <CardHeader>
              <CardTitle>Vulnerabilities</CardTitle>
              <CardDescription>
                Current security vulnerabilities by severity
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-2xl font-bold text-red-500">
                    {securityMetrics.vulnerabilities.critical}
                  </div>
                  <div className="text-sm text-muted-foreground">Critical</div>
                </div>
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-2xl font-bold text-orange-500">
                    {securityMetrics.vulnerabilities.high}
                  </div>
                  <div className="text-sm text-muted-foreground">High</div>
                </div>
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-2xl font-bold text-yellow-500">
                    {securityMetrics.vulnerabilities.medium}
                  </div>
                  <div className="text-sm text-muted-foreground">Medium</div>
                </div>
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-2xl font-bold text-blue-500">
                    {securityMetrics.vulnerabilities.low}
                  </div>
                  <div className="text-sm text-muted-foreground">Low</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Encryption Algorithms */}
          <Card>
            <CardHeader>
              <CardTitle>Active Encryption Algorithms</CardTitle>
              <CardDescription>
                Currently active cryptographic algorithms and their security levels
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {securityMetrics.encryptionAlgorithms.map((algo, index) => {
                  const level = getSecurityLevel(algo.strength)
                  return (
                    <div key={index} className="flex items-center justify-between p-3 border rounded-lg">
                      <div className="flex items-center gap-3">
                        <div className="w-2 h-2 rounded-full bg-green-500"></div>
                        <span className="font-medium">{algo.name}</span>
                      </div>
                      <Badge style={{ backgroundColor: level.color }}>
                        {level.label}
                      </Badge>
                    </div>
                  )
                })}
              </div>
            </CardContent>
          </Card>

          {/* Security Events */}
          <Card>
            <CardHeader>
              <CardTitle>Recent Security Events</CardTitle>
              <CardDescription>
                Latest security-related activities and system events
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {securityMetrics.securityEvents.map((event, index) => (
                  <div key={index} className="flex items-center justify-between p-3 border rounded-lg">
                    <div className="flex items-center gap-3">
                      <div className="w-2 h-2 rounded-full bg-blue-500"></div>
                      <span className="text-sm">{event.message}</span>
                    </div>
                    <span className="text-xs text-muted-foreground">{event.time}</span>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </>
      )}
    </div>
  )
}
