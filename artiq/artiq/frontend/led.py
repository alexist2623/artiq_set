from artiq.experiment import *

class LED(EnvExperiment):
# def __init__(self):
#  print("program run..")
  
 def build(self):
  self.setattr_device("core")
  self.setattr_device("led")

 @kernel
 def run(self):
  self.core.reset()
  self.led.on()
  

#LED()
