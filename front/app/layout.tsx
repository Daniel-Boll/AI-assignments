'use client';

import '@styles/dist.css';
import { Footer, Header } from '@ui/layout';
import { useEffect } from 'react';
import { themeChange } from 'theme-change';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  useEffect(() => themeChange(false));

  return (
    <html lang="en" theme-data="dark">
      <head>
        <title>AI Assignments</title>
      </head>
      <body className="max-w-screen min-h-screen overflow-auto">
        {/* The content here is separate so I can apply the padding and gap for both the Header and the main content but no the Footer */}
        <div className="box-border p-2 flex flex-col h-full gap-2">
          <Header />

          <main className="w-full max-w-full h-full">{children}</main>
        </div>

        <Footer />
      </body>
    </html>
  );
}
