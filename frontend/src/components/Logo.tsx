import React from "react";

interface LogoProps {
  size?: "sm" | "md" | "lg" | "xl";
  showText?: boolean;
  className?: string;
}

const Logo: React.FC<LogoProps> = ({
  size = "md",
  showText = true,
  className = "",
}) => {
  const sizeClasses = {
    sm: "w-8 h-8",
    md: "w-12 h-12",
    lg: "w-16 h-16",
    xl: "w-20 h-20",
  };

  const textSizeClasses = {
    sm: "text-lg",
    md: "text-xl",
    lg: "text-2xl",
    xl: "text-3xl",
  };

  return (
    <div className={`flex items-center space-x-3 ${className}`}>
      {/* Logo Icon */}
      <div className={`${sizeClasses[size]} relative`}>
        {/* Solar Panel Grid */}
        <div className="absolute inset-0 grid grid-cols-2 gap-0.5">
          {Array.from({ length: 4 }).map((_, i) => (
            <div
              key={i}
              className="bg-gradient-to-br from-blue-500 to-blue-600 rounded-sm"
            />
          ))}
        </div>

        {/* Energy Bolt */}
        <div className="absolute inset-0 flex items-center justify-center">
          <svg
            className="w-3/4 h-3/4 text-yellow-400 drop-shadow-sm"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path d="M13 2L3 14h6l-2 8 10-12h-6l2-8z" />
          </svg>
        </div>

        {/* Trading Arrows */}
        <div className="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full flex items-center justify-center">
          <svg
            className="w-2 h-2 text-white"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path d="M7 14l5-5 5 5H7z" />
          </svg>
        </div>
        <div className="absolute -bottom-1 -left-1 w-3 h-3 bg-red-500 rounded-full flex items-center justify-center">
          <svg
            className="w-2 h-2 text-white"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path d="M7 10l5 5 5-5H7z" />
          </svg>
        </div>
      </div>

      {/* Logo Text */}
      {showText && (
        <div className="flex flex-col">
          <span
            className={`font-bold text-gray-900 dark:text-white ${textSizeClasses[size]}`}
          >
            Energy Trading
          </span>
          <span
            className={`text-sm text-gray-600 dark:text-gray-400 -mt-1 ${
              size === "sm" ? "text-xs" : ""
            }`}
          >
            Australia
          </span>
        </div>
      )}
    </div>
  );
};

export default Logo;
