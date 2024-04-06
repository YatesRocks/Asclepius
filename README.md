# Asclepius

This is our submission to Cincy Hacks 2023. Asclepius is a program
for those with Parkinson's, a neurological disorder that can cause
tremors of the hand. Asclepius is intended to monitor the tremors
and provide a dashboard with a histogram, enabling those with
Parkinson's to better understand and mitigate tremors by analyzing
the times & activities in which they are worst affected.

We accomplish this by attaching a gyroscope to the effected limb,
and sending the data through TCP/IP to our secure backend where
it can be parsed & retrieved at any time through our web interface.