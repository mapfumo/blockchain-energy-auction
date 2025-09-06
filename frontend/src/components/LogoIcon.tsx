import React from "react";

interface LogoIconProps {
  size?: number;
  className?: string;
}

const LogoIcon: React.FC<LogoIconProps> = ({ size = 32, className = "" }) => {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 32 32"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      {/* Solar Panel Grid Background */}
      <rect x="2" y="2" width="28" height="28" rx="4" fill="#3B82F6" />

      {/* Solar Panel Cells */}
      <rect x="4" y="4" width="12" height="12" fill="#1D4ED8" />
      <rect x="18" y="4" width="12" height="12" fill="#1D4ED8" />
      <rect x="4" y="18" width="12" height="12" fill="#1D4ED8" />
      <rect x="18" y="18" width="12" height="12" fill="#1D4ED8" />

      {/* Energy Bolt */}
      <path
        d="M16 6L12 18h4l-2 4 4-8h-4l2-4z"
        fill="#FBBF24"
        stroke="#F59E0B"
        strokeWidth="0.5"
      />

      {/* Trading Indicators */}
      <circle cx="26" cy="6" r="2.5" fill="#10B981" />
      <path d="M25 5l1 1-1 1V5z" fill="white" />

      <circle cx="6" cy="26" r="2.5" fill="#EF4444" />
      <path d="M7 27l-1-1 1-1v2z" fill="white" />

      {/* Australian Sun Rays */}
      <path d="M16 2l1 2-1 2-1-2 1-2z" fill="#FBBF24" />
      <path d="M30 16l-2 1-2-1 2-1 2 1z" fill="#FBBF24" />
      <path d="M16 30l-1-2 1-2 1 2-1 2z" fill="#FBBF24" />
      <path d="M2 16l2-1 2 1-2 1-2-1z" fill="#FBBF24" />
    </svg>
  );
};

export default LogoIcon;
