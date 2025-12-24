'use client'

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import { Shield, CheckCircle, AlertTriangle, XCircle } from 'lucide-react'
import { getComplianceStatus } from '@/lib/utils'

interface ComplianceOverviewProps {
  detailed?: boolean
}

export function ComplianceOverview({ detailed = false }: ComplianceOverviewProps) {
  // Mock compliance data - gerçek uygulamada API'den gelecek
  const complianceData = {
    nistStatus: 'compliant' as const,
    fipsValidated: true,
    cnsaCompliant: true,
    overallScore: 98,
    lastAudit: Date.now() - 86400000, // 1 gün önce
    nextAudit: Date.now() + 2592000000, // 30 gün sonra
    standards: [
      { name: 'NIST FIPS 203 (ML-KEM)', status: 'compliant', score: 100 },
      { name: 'NIST FIPS 204 (ML-DSA)', status: 'compliant', score: 100 },
      { name: 'CNSA 2.0 Guidelines', status: 'compliant', score: 95 },
      { name: 'EU NIS2 Directive', status: 'warning', score: 85 },
      { name: 'ISO 27001', status: 'compliant', score: 92 }
    ]
  }

  const overallStatus = getComplianceStatus(complianceData.nistStatus)

  return (
    <div className="space-y-6">
      {/* Overall Compliance Status */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Shield className="h-5 w-5" />
            Compliance Overview
          </CardTitle>
          <CardDescription>
            Current NIST and regulatory compliance status
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-3 gap-4 md:gap-6">
            {/* Overall Status */}
            <div className="space-y-2">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                <span className="text-sm font-medium">Overall Status</span>
                <Badge className={overallStatus.color}>
                  {overallStatus.label}
                </Badge>
              </div>
              <Progress value={complianceData.overallScore} className="h-2" />
              <p className="text-xs text-muted-foreground">
                {complianceData.overallScore}% compliant
              </p>
            </div>

            {/* Last Audit */}
            <div className="space-y-2">
              <span className="text-sm font-medium">Last Security Audit</span>
              <p className="text-sm text-muted-foreground">
                {new Date(complianceData.lastAudit).toLocaleDateString()}
              </p>
              <Badge variant="outline" className="w-fit">
                <CheckCircle className="h-3 w-3 mr-1" />
                Passed
              </Badge>
            </div>

            {/* Next Audit */}
            <div className="space-y-2">
              <span className="text-sm font-medium">Next Audit Due</span>
              <p className="text-sm text-muted-foreground">
                {new Date(complianceData.nextAudit).toLocaleDateString()}
              </p>
              <Badge variant="outline" className="w-fit">
                <AlertTriangle className="h-3 w-3 mr-1" />
                30 days
              </Badge>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Standards Compliance */}
      {detailed && (
        <Card>
          <CardHeader>
            <CardTitle>Standards Compliance</CardTitle>
            <CardDescription>
              Detailed compliance status for each security standard
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {complianceData.standards.map((standard, index) => {
                const status = getComplianceStatus(standard.status)
                const StatusIcon = standard.status === 'compliant' ? CheckCircle :
                                 standard.status === 'warning' ? AlertTriangle : XCircle

                return (
                  <div key={index} className="flex flex-col sm:flex-row sm:items-center justify-between p-4 border rounded-lg gap-3">
                    <div className="flex items-center gap-3">
                      <StatusIcon className={`h-5 w-5 flex-shrink-0 ${
                        standard.status === 'compliant' ? 'text-green-500' :
                        standard.status === 'warning' ? 'text-yellow-500' : 'text-red-500'
                      }`} />
                      <div className="min-w-0 flex-1">
                        <p className="font-medium text-sm sm:text-base truncate">{standard.name}</p>
                        <Badge className={status.color} variant="outline">
                          {status.label}
                        </Badge>
                      </div>
                    </div>
                    <div className="flex items-center gap-3 sm:flex-col sm:items-end">
                      <p className="text-sm font-medium">{standard.score}%</p>
                      <Progress value={standard.score} className="w-16 h-1 sm:w-20" />
                    </div>
                  </div>
                )
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
