import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Download, Zap, Shield, Box } from 'lucide-react';
import { Button } from '../components/ui/Button';
import { Card } from '../components/ui/Card';
import { DownloadButton } from '../components/ui/DownloadButton';
import { SearchBar } from '../components/search/SearchBar';
import { PackageList } from '../components/search/PackageList';

export function Home() {
  const [searchQuery, setSearchQuery] = useState('');

  return (
    <div className="min-h-screen bg-dark-bg text-white overflow-hidden">
      {/* Hero Section */}
      <section className="relative pt-32 pb-20 px-6">
        <div className="absolute inset-0 bg-blue-600/10 blur-[100px] -z-10 rounded-full transform -translate-y-1/2" />
        <div className="max-w-7xl mx-auto text-center">
          <motion.h1 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-5xl md:text-7xl font-bold mb-6 bg-gradient-to-r from-blue-400 via-cyan-400 to-teal-400 bg-clip-text text-transparent"
          >
            The Modern Package Manager<br />for Windows
          </motion.h1>
          <motion.p 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="text-xl text-slate-400 mb-10 max-w-2xl mx-auto"
          >
            Blazing fast, secure, and built with Rust. Say goodbye to slow installs and complex setups.
          </motion.p>
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="flex flex-col sm:flex-row gap-4 justify-center"
          >
            <DownloadButton className="text-lg px-8" />
            <Button variant="secondary" className="text-lg px-8" onClick={() => document.getElementById('packages').scrollIntoView({ behavior: 'smooth' })}>
              View Packages
            </Button>
          </motion.div>
        </div>
      </section>

      {/* Search/Packages Section */}
      <section id="packages" className="py-20 px-6 bg-dark-surface/30">
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold mb-4">Explore Packages</h2>
            <p className="text-slate-400">Find and install your favorite tools in seconds.</p>
          </div>
          <SearchBar value={searchQuery} onChange={setSearchQuery} />
          <PackageList searchQuery={searchQuery} />
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 px-6 bg-dark-surface/50">
        <div className="max-w-7xl mx-auto">
          <h2 className="text-3xl font-bold text-center mb-12">Why Paketron?</h2>
          <div className="grid md:grid-cols-3 gap-8">
            <FeatureCard 
              icon={<Zap className="w-8 h-8 text-yellow-400" />}
              title="Blazing Fast"
              description="Written in Rust for instant startup and parallel downloads. No runtime required."
            />
            <FeatureCard 
              icon={<Shield className="w-8 h-8 text-green-400" />}
              title="Secure by Default"
              description="Every package is verified with SHA256 checksums. Your system safety is our priority."
            />
            <FeatureCard 
              icon={<Box className="w-8 h-8 text-blue-400" />}
              title="Simple & Clean"
              description="No complex dependencies. Just a single binary that does one thing well."
            />
          </div>
        </div>
      </section>

      {/* Comparison Section */}
      <section className="py-20 px-6">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-3xl font-bold text-center mb-12">Better than the Rest</h2>
          <div className="grid md:grid-cols-2 gap-8">
            <Card className="border-blue-500/20">
              <h3 className="text-xl font-bold mb-4 text-blue-400">Paketron</h3>
              <ul className="space-y-3 text-slate-300">
                <li className="flex items-center gap-2">✅ Instant startup (&lt; 200ms)</li>
                <li className="flex items-center gap-2">✅ No external dependencies</li>
                <li className="flex items-center gap-2">✅ Parallel downloads</li>
                <li className="flex items-center gap-2">✅ Modern CLI experience</li>
              </ul>
            </Card>
            <Card className="opacity-75 grayscale hover:grayscale-0 transition-all">
              <h3 className="text-xl font-bold mb-4 text-slate-400">Others (Chocolatey)</h3>
              <ul className="space-y-3 text-slate-400">
                <li className="flex items-center gap-2">❌ Requires .NET Runtime</li>
                <li className="flex items-center gap-2">❌ Slower startup times</li>
                <li className="flex items-center gap-2">❌ Complex enterprise features</li>
                <li className="flex items-center gap-2">❌ Legacy codebase</li>
              </ul>
            </Card>
          </div>
        </div>
      </section>
    </div>
  );
}

function FeatureCard({ icon, title, description }) {
  return (
    <Card className="text-center hover:bg-slate-800/80">
      <div className="bg-slate-900/50 w-16 h-16 rounded-2xl flex items-center justify-center mx-auto mb-6">
        {icon}
      </div>
      <h3 className="text-xl font-bold mb-3">{title}</h3>
      <p className="text-slate-400">{description}</p>
    </Card>
  );
}
