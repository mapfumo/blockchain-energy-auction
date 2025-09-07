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
      {/* Logo Icon - Using Favicon */}
      <div className={`${sizeClasses[size]} relative`}>
        <img
          src="/favicon.ico?v=2"
          alt="Energy Trading Logo"
          className="w-full h-full object-contain"
        />
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
