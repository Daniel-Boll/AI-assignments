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
      <body className="w-screen h-screen">
        {/* The content here is separate so I can apply the padding and gap for both the Header and the main content but no the Footer */}
        <div className="p-2 grid gap-2">
          <Header />

          <main>{children}</main>
        </div>

        <Footer />
      </body>
    </html>
  );
}
