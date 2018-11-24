import { TestBed } from "@angular/core/testing";

import { TokenModelService } from "./token-model.service";

describe("TokenService", () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it("should be created", () => {
    const service: TokenModelService = TestBed.get(TokenModelService);
    expect(service).toBeTruthy();
  });
});
