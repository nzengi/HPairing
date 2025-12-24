import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'HPair - Enterprise Compliance Dashboard',
  description: 'Post-Quantum Secure Multi-Party Communication Platform - NIST Compliant',
  keywords: ['quantum', 'cryptography', 'enterprise', 'compliance', 'security'],
  authors: [{ name: 'HPair Team' }],
  robots: 'noindex, nofollow', // Enterprise security - no public indexing
}

export const viewport = {
  width: 'device-width',
  initialScale: 1,
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="h-full">
      <head>
        <meta name="theme-color" content="#1a365d" />
        <link rel="icon" href="/favicon.ico" />
      </head>
      <body className={`${inter.className} h-full bg-background text-foreground overflow-x-hidden`}>
        <div className="min-h-screen flex flex-col">
          {children}
        </div>
      </body>
    </html>
  )
}
