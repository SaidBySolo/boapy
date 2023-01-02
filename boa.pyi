class JSExeption(Exception):
    """
    JavaScript exception
    """

class BoaPyContext:
    def execute(self, code: str) -> str:
        """
        Execute the code given in the context
        """
