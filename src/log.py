"""
Provide a way to enable logging by setting LOG_LEVEL environment variable
"""
import logging
import os

# Set default logging level to WARNING if LOG_LEVEL is not set
log_level = os.getenv("LOG_LEVEL", "INFO").upper()
logger = logging.getLogger(__package__)
logger.setLevel(level=getattr(logging, log_level, logging.WARNING))
# Setup logging to stdout
logging.basicConfig(format='%(levelname)s [%(filename)s:%(lineno)d] %(message)s')
    
log = logging.getLogger(__package__)