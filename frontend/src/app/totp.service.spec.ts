import { TestBed } from '@angular/core/testing';

import { TotpService } from './totp.service';

describe('TotpService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: TotpService = TestBed.get(TotpService);
    expect(service).toBeTruthy();
  });
});
