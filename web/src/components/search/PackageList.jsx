import React from 'react';
import { Download, Star, Terminal } from 'lucide-react';
import { Card } from '../ui/Card';
import { Button } from '../ui/Button';

export function PackageList({ searchQuery }) {
  // Mock Data
  const packages = [
    { name: '7zip', version: '23.01', description: 'High compression file archiver', downloads: '2.4M' },
    { name: 'vscode', version: '1.85.0', description: 'Code editing. Redefined.', downloads: '15M' },
    { name: 'rust', version: '1.75.0', description: 'A language empowering everyone to build reliable and efficient software.', downloads: '8.2M' },
    { name: 'git', version: '2.43.0', description: 'Distributed version control system', downloads: '12M' },
    { name: 'node', version: '20.10.0', description: 'JavaScript runtime built on Chrome\'s V8 engine', downloads: '10M' },
    { name: 'python', version: '3.12.1', description: 'Programming language that lets you work quickly', downloads: '9.5M' },
  ];

  const filteredPackages = packages.filter(pkg => 
    pkg.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
    pkg.description.toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (filteredPackages.length === 0) {
    return (
      <div className="text-center py-12 text-slate-500">
        No packages found matching "{searchQuery}"
      </div>
    );
  }

  return (
    <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6 mt-8">
      {filteredPackages.map((pkg) => (
        <Card key={pkg.name} className="hover:border-blue-500/50 group">
          <div className="flex justify-between items-start mb-4">
            <div className="p-3 bg-slate-900 rounded-xl group-hover:bg-blue-500/10 transition-colors">
              <Terminal className="w-6 h-6 text-blue-400" />
            </div>
            <div className="flex items-center gap-1 text-slate-500 text-sm">
              <Download className="w-4 h-4" />
              {pkg.downloads}
            </div>
          </div>
          <h3 className="text-xl font-bold mb-1">{pkg.name}</h3>
          <p className="text-slate-500 text-sm mb-4 line-clamp-2">{pkg.description}</p>
          <div className="flex items-center justify-between mt-auto pt-4 border-t border-slate-700/50">
            <span className="text-xs font-mono text-slate-400 bg-slate-900 px-2 py-1 rounded">v{pkg.version}</span>
            <Button variant="secondary" className="text-xs px-3 py-1.5 h-auto">
              Install
            </Button>
          </div>
        </Card>
      ))}
    </div>
  );
}
