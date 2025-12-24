"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  FileText,
  Search,
  Filter,
  Download,
  Eye,
  User,
  Users,
  Shield,
  MessageSquare,
} from "lucide-react";
import { AuditLog } from "@/types";
import { formatTimestamp } from "@/lib/utils";

export function AuditLogs() {
  const [searchTerm, setSearchTerm] = useState("");
  const [filterType, setFilterType] = useState<string>("all");
  const [filterUser, setFilterUser] = useState<string>("all");

  // Mock audit logs - gerçek uygulamada API'den gelecek
  const [auditLogs] = useState<AuditLog[]>([
    {
      id: "log_001",
      timestamp: Date.now() - 3600000, // 1 hour ago
      action: "Group Created",
      userId: "user_001",
      userName: "Alice Johnson",
      resourceType: "group",
      resourceId: "group_001",
      details: { groupName: "Executive Board", participants: 3 },
      ipAddress: "192.168.1.100",
    },
    {
      id: "log_002",
      timestamp: Date.now() - 1800000, // 30 min ago
      action: "Message Sent",
      userId: "user_002",
      userName: "Bob Smith",
      resourceType: "message",
      resourceId: "msg_001",
      details: { groupId: "group_001", messageLength: 256 },
      ipAddress: "192.168.1.101",
    },
    {
      id: "log_003",
      timestamp: Date.now() - 900000, // 15 min ago
      action: "Key Rotation",
      userId: "system",
      userName: "System",
      resourceType: "system",
      resourceId: "group_001",
      details: { algorithm: "ML-KEM-1024", reason: "scheduled" },
      ipAddress: "127.0.0.1",
    },
    {
      id: "log_004",
      timestamp: Date.now() - 720000, // 12 min ago
      action: "User Joined Group",
      userId: "user_003",
      userName: "Carol Davis",
      resourceType: "group",
      resourceId: "group_001",
      details: { role: "user" },
      ipAddress: "192.168.1.102",
    },
    {
      id: "log_005",
      timestamp: Date.now() - 300000, // 5 min ago
      action: "Compliance Audit",
      userId: "system",
      userName: "System",
      resourceType: "system",
      resourceId: "global",
      details: { status: "passed", score: 98, standards: ["NIST", "CNSA"] },
      ipAddress: "127.0.0.1",
    },
  ]);

  const filteredLogs = auditLogs.filter((log) => {
    const matchesSearch =
      log.action.toLowerCase().includes(searchTerm.toLowerCase()) ||
      log.userName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      log.resourceId.toLowerCase().includes(searchTerm.toLowerCase());

    const matchesType = filterType === "all" || log.resourceType === filterType;
    const matchesUser = filterUser === "all" || log.userId === filterUser;

    return matchesSearch && matchesType && matchesUser;
  });

  const getActionIcon = (action: string) => {
    if (action.includes("Group")) return Users;
    if (action.includes("Message")) return MessageSquare;
    if (action.includes("Key") || action.includes("Audit")) return Shield;
    return User;
  };

  const getActionColor = (action: string) => {
    if (action.includes("Created") || action.includes("Joined"))
      return "text-green-600";
    if (action.includes("Sent") || action.includes("Rotation"))
      return "text-blue-600";
    if (action.includes("Audit")) return "text-purple-600";
    return "text-gray-600";
  };

  const getResourceTypeBadge = (type: string) => {
    const colors = {
      group: "bg-blue-100 text-blue-800",
      message: "bg-green-100 text-green-800",
      user: "bg-purple-100 text-purple-800",
      system: "bg-orange-100 text-orange-800",
    };
    return colors[type as keyof typeof colors] || "bg-gray-100 text-gray-800";
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Security Audit Logs</h2>
          <p className="text-muted-foreground">
            Complete audit trail of all security-related activities
          </p>
        </div>
        <Button>
          <Download className="h-4 w-4 mr-2" />
          Export Logs
        </Button>
      </div>

      {/* Filters */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex flex-col md:flex-row gap-4">
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                <input
                  type="text"
                  placeholder="Search logs..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-3 py-2 border border-input rounded-md bg-background"
                />
              </div>
            </div>
            <div className="flex gap-2">
              <select
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
                className="px-3 py-2 border border-input rounded-md bg-background"
              >
                <option value="all">All Types</option>
                <option value="group">Groups</option>
                <option value="message">Messages</option>
                <option value="user">Users</option>
                <option value="system">System</option>
              </select>
              <select
                value={filterUser}
                onChange={(e) => setFilterUser(e.target.value)}
                className="px-3 py-2 border border-input rounded-md bg-background"
              >
                <option value="all">All Users</option>
                <option value="user_001">Alice Johnson</option>
                <option value="user_002">Bob Smith</option>
                <option value="user_003">Carol Davis</option>
                <option value="system">System</option>
              </select>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Audit Summary */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-4 gap-4 md:gap-6">
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <FileText className="h-6 w-6 sm:h-8 sm:w-8 text-blue-500 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="text-xl sm:text-2xl font-bold">
                  {auditLogs.length}
                </div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">
                  Total Events
                </p>
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
                  {
                    auditLogs.filter((log) => log.action.includes("Audit"))
                      .length
                  }
                </div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">
                  Security Events
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <Users className="h-6 w-6 sm:h-8 sm:w-8 text-purple-500 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="text-xl sm:text-2xl font-bold">
                  {
                    auditLogs.filter((log) => log.resourceType === "group")
                      .length
                  }
                </div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">
                  Group Events
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <MessageSquare className="h-6 w-6 sm:h-8 sm:w-8 text-orange-500 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="text-xl sm:text-2xl font-bold">
                  {
                    auditLogs.filter((log) => log.resourceType === "message")
                      .length
                  }
                </div>
                <p className="text-xs sm:text-sm text-muted-foreground truncate">
                  Message Events
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Audit Logs Table */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Audit Events</CardTitle>
          <CardDescription>
            Detailed log of all security and operational activities
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="text-left p-4 font-medium">Timestamp</th>
                  <th className="text-left p-4 font-medium">Action</th>
                  <th className="text-left p-4 font-medium">User</th>
                  <th className="text-left p-4 font-medium">Resource</th>
                  <th className="text-left p-4 font-medium">Details</th>
                  <th className="text-left p-4 font-medium">IP Address</th>
                  <th className="text-left p-4 font-medium">Actions</th>
                </tr>
              </thead>
              <tbody>
                {filteredLogs.map((log) => {
                  const ActionIcon = getActionIcon(log.action);
                  const actionColor = getActionColor(log.action);

                  return (
                    <tr key={log.id} className="border-b hover:bg-muted/50">
                      <td className="p-4 font-mono text-xs">
                        {formatTimestamp(log.timestamp)}
                      </td>
                      <td className="p-4">
                        <div className="flex items-center gap-2">
                          <ActionIcon className={`h-4 w-4 ${actionColor}`} />
                          <span className="font-medium">{log.action}</span>
                        </div>
                      </td>
                      <td className="p-4">
                        <div className="flex items-center gap-2">
                          <User className="h-4 w-4 text-muted-foreground" />
                          {log.userName}
                        </div>
                      </td>
                      <td className="p-4">
                        <Badge
                          className={getResourceTypeBadge(log.resourceType)}
                        >
                          {log.resourceType}
                        </Badge>
                        <div className="text-xs text-muted-foreground mt-1">
                          {log.resourceId}
                        </div>
                      </td>
                      <td className="p-4">
                        <div className="max-w-xs truncate">
                          {Object.entries(log.details).map(([key, value]) => (
                            <div key={key} className="text-xs">
                              <span className="font-medium">{key}:</span>{" "}
                              {String(value)}
                            </div>
                          ))}
                        </div>
                      </td>
                      <td className="p-4 font-mono text-xs">{log.ipAddress}</td>
                      <td className="p-4">
                        <button className="p-2 hover:bg-accent rounded">
                          <Eye className="h-4 w-4" />
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>

          {filteredLogs.length === 0 && (
            <div className="text-center py-8 text-muted-foreground">
              No audit logs found matching your filters.
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
