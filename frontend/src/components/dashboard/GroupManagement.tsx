'use client'

import { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Users, Plus, Shield, Clock, HardDrive, MessageSquare, Eye } from 'lucide-react'
import { Group, Participant } from '@/types'
import { formatTimestamp, formatBytes, getComplianceStatus, getSecurityLevel } from '@/lib/utils'

export function GroupManagement() {
  const [groups, setGroups] = useState<Group[]>([
    {
      id: 'group_001',
      name: 'Executive Board',
      participants: [
        { id: 'user_001', name: 'Alice Johnson', role: 'admin', joinedAt: Date.now() - 86400000 * 30, lastSeen: Date.now() - 3600000 },
        { id: 'user_002', name: 'Bob Smith', role: 'user', joinedAt: Date.now() - 86400000 * 25, lastSeen: Date.now() - 1800000 },
        { id: 'user_003', name: 'Carol Davis', role: 'user', joinedAt: Date.now() - 86400000 * 20, lastSeen: Date.now() - 7200000 }
      ],
      createdAt: Date.now() - 86400000 * 30,
      lastActivity: Date.now() - 1800000,
      messageCount: 1247,
      encryptionLevel: 'quantum',
      complianceStatus: 'compliant',
      storageSize: 1024 * 1024 * 50 // 50MB
    },
    {
      id: 'group_002',
      name: 'Finance Team',
      participants: [
        { id: 'user_004', name: 'David Wilson', role: 'admin', joinedAt: Date.now() - 86400000 * 15, lastSeen: Date.now() - 900000 },
        { id: 'user_005', name: 'Eva Brown', role: 'user', joinedAt: Date.now() - 86400000 * 12, lastSeen: Date.now() - 3600000 }
      ],
      createdAt: Date.now() - 86400000 * 15,
      lastActivity: Date.now() - 900000,
      messageCount: 892,
      encryptionLevel: 'quantum',
      complianceStatus: 'compliant',
      storageSize: 1024 * 1024 * 25 // 25MB
    },
    {
      id: 'group_003',
      name: 'IT Security',
      participants: [
        { id: 'user_006', name: 'Frank Miller', role: 'admin', joinedAt: Date.now() - 86400000 * 7, lastSeen: Date.now() - 300000 },
        { id: 'user_007', name: 'Grace Lee', role: 'user', joinedAt: Date.now() - 86400000 * 5, lastSeen: Date.now() - 1800000 },
        { id: 'user_008', name: 'Henry Taylor', role: 'auditor', joinedAt: Date.now() - 86400000 * 3, lastSeen: Date.now() - 7200000 }
      ],
      createdAt: Date.now() - 86400000 * 7,
      lastActivity: Date.now() - 300000,
      messageCount: 567,
      encryptionLevel: 'quantum',
      complianceStatus: 'warning',
      storageSize: 1024 * 1024 * 75 // 75MB
    }
  ])

  const [selectedGroup, setSelectedGroup] = useState<Group | null>(null)
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [newGroupName, setNewGroupName] = useState('')
  const [newGroupParticipants, setNewGroupParticipants] = useState('')

  const handleCreateGroup = () => {
    if (!newGroupName.trim()) return

    const participants = newGroupParticipants
      .split(',')
      .map(name => name.trim())
      .filter(name => name.length > 0)
      .map((name, index) => ({
        id: `user_${Date.now()}_${index}`,
        name,
        role: index === 0 ? 'admin' as const : 'user' as const,
        joinedAt: Date.now(),
        lastSeen: Date.now()
      }))

    const newGroup: Group = {
      id: `group_${Date.now()}`,
      name: newGroupName,
      participants,
      createdAt: Date.now(),
      lastActivity: Date.now(),
      messageCount: 0,
      encryptionLevel: 'quantum',
      complianceStatus: 'compliant',
      storageSize: 0
    }

    setGroups(prev => [...prev, newGroup])
    setNewGroupName('')
    setNewGroupParticipants('')
    setShowCreateForm(false)
  }

  const complianceStatus = getComplianceStatus('compliant')
  const securityLevel = getSecurityLevel('quantum')

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Secure Group Management</h2>
          <p className="text-muted-foreground">
            Manage post-quantum encrypted communication groups
          </p>
        </div>
        <button
          onClick={() => setShowCreateForm(!showCreateForm)}
          className="inline-flex items-center px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
        >
          <Plus className="h-4 w-4 mr-2" />
          Create Group
        </button>
      </div>

      {/* Create Group Form */}
      {showCreateForm && (
        <Card>
          <CardHeader>
            <CardTitle>Create New Secure Group</CardTitle>
            <CardDescription>
              Create a new post-quantum encrypted communication group with NIST-compliant security.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Group Name</label>
                <input
                  type="text"
                  placeholder="e.g., Executive Board"
                  value={newGroupName}
                  onChange={(e) => setNewGroupName(e.target.value)}
                  className="w-full px-3 py-2 border border-input rounded-md bg-background"
                />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Initial Participants</label>
                <input
                  type="text"
                  placeholder="e.g., Alice Johnson, Bob Smith, Carol Davis"
                  value={newGroupParticipants}
                  onChange={(e) => setNewGroupParticipants(e.target.value)}
                  className="w-full px-3 py-2 border border-input rounded-md bg-background"
                />
                <p className="text-xs text-muted-foreground">
                  Separate names with commas. First participant will be admin.
                </p>
              </div>
              <div className="flex gap-2">
                <button
                  onClick={handleCreateGroup}
                  className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                >
                  Create Group
                </button>
                <button
                  onClick={() => setShowCreateForm(false)}
                  className="px-4 py-2 border border-input rounded-md hover:bg-accent"
                >
                  Cancel
                </button>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Groups Overview */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-3 gap-4 md:gap-6">
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <Users className="h-6 w-6 sm:h-8 sm:w-8 text-blue-500 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="text-xl sm:text-2xl font-bold">{groups.length}</div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">Total Groups</p>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <Shield className="h-6 w-6 sm:h-8 sm:w-8 text-green-500 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="text-xl sm:text-2xl font-bold">
                  {groups.filter(g => g.complianceStatus === 'compliant').length}
                </div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">Compliant</p>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <MessageSquare className="h-6 w-6 sm:h-8 sm:w-8 text-purple-500 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="text-xl sm:text-2xl font-bold truncate">
                  {groups.reduce((sum, g) => sum + g.messageCount, 0).toLocaleString()}
                </div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">Total Messages</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Groups Table */}
      <Card>
        <CardHeader>
          <CardTitle>Active Groups</CardTitle>
          <CardDescription>
            Manage and monitor your secure communication groups
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="text-left p-4 font-medium">Group Name</th>
                  <th className="text-left p-4 font-medium">Participants</th>
                  <th className="text-left p-4 font-medium">Security Level</th>
                  <th className="text-left p-4 font-medium">Compliance</th>
                  <th className="text-left p-4 font-medium">Storage</th>
                  <th className="text-left p-4 font-medium">Last Activity</th>
                  <th className="text-left p-4 font-medium">Actions</th>
                </tr>
              </thead>
              <tbody>
                {groups.map((group) => {
                  const compliance = getComplianceStatus(group.complianceStatus)
                  const security = getSecurityLevel(group.encryptionLevel)

                  return (
                    <tr key={group.id} className="border-b hover:bg-muted/50">
                      <td className="p-4 font-medium">{group.name}</td>
                      <td className="p-4">
                        <div className="flex items-center gap-2">
                          <Users className="h-4 w-4" />
                          {group.participants.length} members
                        </div>
                      </td>
                      <td className="p-4">
                        <Badge style={{ backgroundColor: security.color }}>
                          {security.label}
                        </Badge>
                      </td>
                      <td className="p-4">
                        <Badge variant={compliance.variant === 'safe' ? 'compliance-safe' :
                                       compliance.variant === 'warning' ? 'compliance-warning' :
                                       compliance.variant === 'critical' ? 'compliance-critical' : 'compliance-pending'}>
                          {compliance.label}
                        </Badge>
                      </td>
                      <td className="p-4">
                        <div className="flex items-center gap-2">
                          <HardDrive className="h-4 w-4" />
                          {formatBytes(group.storageSize)}
                        </div>
                      </td>
                      <td className="p-4">
                        <div className="flex items-center gap-2">
                          <Clock className="h-4 w-4" />
                          {formatTimestamp(group.lastActivity)}
                        </div>
                      </td>
                      <td className="p-4">
                        <button
                          onClick={() => setSelectedGroup(group)}
                          className="px-3 py-1 border border-input rounded hover:bg-accent text-sm"
                        >
                          <Eye className="h-4 w-4 inline mr-1" />
                          View
                        </button>
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      {/* Group Details */}
      {selectedGroup && (
        <Card>
          <CardHeader>
            <CardTitle>{selectedGroup.name}</CardTitle>
            <CardDescription>
              Group ID: {selectedGroup.id}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-6">
              {/* Group Stats */}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-xl font-bold">{selectedGroup.participants.length}</div>
                  <div className="text-sm text-muted-foreground">Participants</div>
                </div>
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-xl font-bold">{selectedGroup.messageCount}</div>
                  <div className="text-sm text-muted-foreground">Messages</div>
                </div>
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-xl font-bold">{formatBytes(selectedGroup.storageSize)}</div>
                  <div className="text-sm text-muted-foreground">Storage</div>
                </div>
                <div className="text-center p-4 border rounded-lg">
                  <div className="text-xl font-bold">
                    {formatTimestamp(selectedGroup.createdAt)}
                  </div>
                  <div className="text-sm text-muted-foreground">Created</div>
                </div>
              </div>

              {/* Participants */}
              <div>
                <h3 className="text-lg font-semibold mb-4">Participants</h3>
                <div className="space-y-2">
                  {selectedGroup.participants.map((participant) => (
                    <div key={participant.id} className="flex items-center justify-between p-3 border rounded-lg">
                      <div className="flex items-center gap-3">
                        <div className="w-8 h-8 rounded-full bg-primary flex items-center justify-center text-primary-foreground text-sm font-medium">
                          {participant.name.split(' ').map(n => n[0]).join('')}
                        </div>
                        <div>
                          <p className="font-medium">{participant.name}</p>
                          <p className="text-sm text-muted-foreground">
                            Joined {formatTimestamp(participant.joinedAt)}
                          </p>
                        </div>
                      </div>
                      <Badge variant="outline">
                        {participant.role}
                      </Badge>
                    </div>
                  ))}
                </div>
              </div>

              <div className="flex justify-end">
                <button
                  onClick={() => setSelectedGroup(null)}
                  className="px-4 py-2 border border-input rounded-md hover:bg-accent"
                >
                  Close
                </button>
              </div>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
